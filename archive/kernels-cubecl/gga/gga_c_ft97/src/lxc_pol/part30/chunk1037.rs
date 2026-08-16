//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1037/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1037<F: Float>(t1113: F, t7464: F, t123607: F, t13411: F, t13520: F, t140932: F, t142832: F, t150546: F, t150727: F, t150736: F, t150764: F, t150770: F, t17807: F, t17859: F, t27495: F, t27529: F, t27552: F, t27596: F, t27717: F, t33383: F, t33394: F, t33434: F, t33436: F, t33437: F, t36796: F, t3774: F, t66076: F, t66382: F, t683: F, t709: F, t7853: F) -> F {
    let t150808 = t7464 * t1113;
    let t150831 = -F::cast_from(0.1721820212247325051e-5_f64) * t3774 * t150736 - F::cast_from(0.25845121844514357744e-4_f64) * t13520 * t150727 + F::cast_from(0.93911185864662097827e-1_f64) * t33434 * t150546 * t33437 - F::cast_from(0.22979081259345929704e-6_f64) * t17807 * t33394 * t27596 - F::cast_from(0.39525571512470170088e-4_f64) * t36796 * t142832 * t150808 * t709 + F::cast_from(0.22979081259345929704e-6_f64) * t66076 * t33394 * t27552 - F::cast_from(0.45958162518691859409e-6_f64) * t123607 * t33394 * t27529 + F::cast_from(0.70433389398496573372e-1_f64) * t140932 * t33436 * t683 * t17859 - F::cast_from(0.47419045182740103902e-1_f64) * t27717 * t7853 * t27495 - F::cast_from(0.25845121844514357744e-4_f64) * t33383 * t150764 + F::cast_from(0.60102574844279699039e-6_f64) * t13411 * t66382 * t150770;
    t150831
}
