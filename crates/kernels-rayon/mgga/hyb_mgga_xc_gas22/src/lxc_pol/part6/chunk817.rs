//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 817/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk817(t1145: f64, t4576: f64, t1117: f64, t1134: f64, t1144: f64, t1149: f64, t1167: f64, t1169: f64, t2903: f64, t2922: f64, t2927: f64, t4541: f64, t4545: f64, t4550: f64, t4553: f64, t4556: f64, t4559: f64, t4562: f64, t4565: f64, t4568: f64, t4571: f64, t4574: f64, t510: f64, t513: f64, t518: f64, t538: f64) -> (f64, f64) {
    let t4577 = t1145 * t4576;
    let t4582 = 2.0_f64 * t1117 * t4565 + 6.0_f64 * t1134 * t4559 + 3.0_f64 * t1144 * t4541 - 3.0_f64 * t1149 * t4545 + t1167 * t4541 - t1169 * t4545 + 30.0_f64 * t2903 * t4553 - 36.0_f64 * t2922 * t4577 - 4.0_f64 * t2927 * t4577 + 6.0_f64 * t510 * t4550 + 42.0_f64 * t518 * t4556 - 6.0_f64 * t518 * t4562 - 2.0_f64 * t510 * t4568 + 2.0_f64 * t4571 * t513 + t4574 * t538;
    (t4577, t4582)
}
