//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3657/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3657(t16943: f64, t5063: f64, t43748: f64, t6439: f64, t12238: f64, t6471: f64, t20448: f64, t3379: f64, t1196: f64, t3520: f64, t3523: f64, t68795: f64) -> (f64, f64, f64, f64, f64) {
    let t69101 = 2.0_f64 * t5063 * t16943;
    let t69103 = 2.0_f64 * t43748 * t6439;
    let t69105 = 1.0_f64 * t12238 * t6471;
    let t69107 = 2.0_f64 * t3379 * t20448;
    let t69111 = 0.34631718211362927518e2_f64 * t1196 * t3520 * t68795 * t3523;
    (t69101, t69103, t69105, t69107, t69111)
}
