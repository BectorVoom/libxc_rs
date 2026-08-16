//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 595/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk595(t3251: f64, t546: f64, t1474: f64, t3255: f64, t1098: f64, t1479: f64, t1484: f64, t461: f64, t531: f64) -> (f64, f64, f64, f64, f64) {
    let t3743 = 0.21901432222222222222e-3_f64 * t3251 * t546;
    let t3744 = t3255 * t1474;
    let t3746 = t1098 * t1479;
    let t3748 = t1098 * t1484;
    let t3750 = t461 * t531;
    let t3751 = 1.0_f64 / t3750;
    (t3743, t3744, t3746, t3748, t3751)
}
