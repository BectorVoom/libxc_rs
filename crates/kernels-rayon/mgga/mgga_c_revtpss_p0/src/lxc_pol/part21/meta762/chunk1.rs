//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2703/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2703(t105: f64, t4283: f64, t588: f64, t100: f64, t10217: f64, t10236: f64, t10243: f64, t10247: f64, t10250: f64, t10251: f64, t108: f64, t13479: f64, t13482: f64, t1505: f64, t1507: f64, t22: f64, t2344: f64, t2357: f64, t4269: f64, t4270: f64, t4274: f64, t4279: f64, t580: f64, t656: f64, t661: f64, t97: f64) -> f64 {
    let t49745 = 20.0_f64 * t105 * t4283 * t588;
    let t49760 = -25.0_f64 / 9.0_f64 * t1507 * t10251 - 2200.0_f64 / 81.0_f64 * t10217 * t1505 - 25.0_f64 / 3.0_f64 * t656 * t13482 - 10.0_f64 * t97 * t100 * t22 + 50.0_f64 / 81.0_f64 * t1507 * t10243 + 10.0_f64 * t105 * t108 * t22 - 10.0_f64 / 3.0_f64 * t105 * t2357 * t580 * t661 - t49745 + 400.0_f64 / 27.0_f64 * t2344 * t4270 + 200.0_f64 / 9.0_f64 * t2344 * t4274 - 50.0_f64 / 9.0_f64 * t656 * t13479 + 10.0_f64 / 9.0_f64 * t97 * t4269 * t10236 - 50.0_f64 / 9.0_f64 * t1507 * t10247 + 10.0_f64 / 9.0_f64 * t105 * t4279 * t10250;
    t49760
}
