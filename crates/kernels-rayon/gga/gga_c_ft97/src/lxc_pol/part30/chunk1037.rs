//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1037/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1037(t1113: f64, t7464: f64, t123607: f64, t13411: f64, t13520: f64, t140932: f64, t142832: f64, t150546: f64, t150727: f64, t150736: f64, t150764: f64, t150770: f64, t17807: f64, t17859: f64, t27495: f64, t27529: f64, t27552: f64, t27596: f64, t27717: f64, t33383: f64, t33394: f64, t33434: f64, t33436: f64, t33437: f64, t36796: f64, t3774: f64, t66076: f64, t66382: f64, t683: f64, t709: f64, t7853: f64) -> f64 {
    let t150808 = t7464 * t1113;
    let t150831 = -0.1721820212247325051e-5_f64 * t3774 * t150736 - 0.25845121844514357744e-4_f64 * t13520 * t150727 + 0.93911185864662097827e-1_f64 * t33434 * t150546 * t33437 - 0.22979081259345929704e-6_f64 * t17807 * t33394 * t27596 - 0.39525571512470170088e-4_f64 * t36796 * t142832 * t150808 * t709 + 0.22979081259345929704e-6_f64 * t66076 * t33394 * t27552 - 0.45958162518691859409e-6_f64 * t123607 * t33394 * t27529 + 0.70433389398496573372e-1_f64 * t140932 * t33436 * t683 * t17859 - 0.47419045182740103902e-1_f64 * t27717 * t7853 * t27495 - 0.25845121844514357744e-4_f64 * t33383 * t150764 + 0.60102574844279699039e-6_f64 * t13411 * t66382 * t150770;
    t150831
}
