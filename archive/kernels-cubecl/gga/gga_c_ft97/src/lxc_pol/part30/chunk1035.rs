//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1035/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1035<F: Float>(t27596: F, t33384: F, t1095: F, t218: F, t51: F, t6783: F, t27703: F, t123124: F, t33365: F, t35466: F, t666: F, t7477: F) -> (F, F, F, F, F, F) {
    let t150764 = t33384 * t27596;
    let t150770 = t6783 * t51 * t218 * t1095;
    let t150773 = t27703 * t6783;
    let t150776 = t123124 * t33365;
    let t150786 = t35466 * t666;
    let t150787 = t7477 * t150786;
    (t150764, t150770, t150773, t150776, t150786, t150787)
}
