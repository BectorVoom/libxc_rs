//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 621/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk621(t3330: f64, t6638: f64, t143: f64, t6432: f64, t3399: f64, t3400: f64, t6272: f64, t1154: f64, t1646: f64, t5153: f64, t3410: f64, t1155: f64, t6276: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6640 = 2.0_f64 * t3330 * t6638;
    let t6641 = t6432 * t143;
    let t6661 = t3399 * t3400 * t6272;
    let t6665 = t1154 * t5153 * t1646;
    let t6669 = t1154 * t3410 * t6272;
    let t6673 = t1154 * t1155 * t6276;
    (t6640, t6641, t6661, t6665, t6669, t6673)
}
