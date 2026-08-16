//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1080/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1080(t3: f64, t3160: f64, t1238: f64, t8561: f64, t3282: f64, t8492: f64, t2033: f64, t3938: f64, t10325: f64, t688: f64, t3150: f64, t4089: f64, t6227: f64, t6468: f64, t6471: f64, t6485: f64, t677: f64, t684: f64, t687: f64, t8560: f64, t8575: f64, t8577: f64, t8579: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10486 = t3160 * t3;
    let t10490 = t8561 * t1238;
    let t10494 = t3282 * t3;
    let t10498 = t8492 * t1238;
    let t10505 = t2033 * t3938;
    let t10509 = t688 * t10325;
    let t10513 = -3.0_f64 / 64.0_f64 * t677 * t4089 - t6468 + t6485 / 96.0_f64 + t6227 / 96.0_f64 + t6471 / 288.0_f64 + t684 * t3150 * t10486 / 16.0_f64 - t684 * t687 * t10490 / 32.0_f64 + t684 * t3150 * t10494 / 16.0_f64 - t8560 - t684 * t687 * t10498 / 32.0_f64 + t8575 / 144.0_f64 + t8577 / 48.0_f64 + t8579 / 16.0_f64 - t684 * t687 * t10505 / 64.0_f64 - t684 * t687 * t10509 / 64.0_f64;
    (t10486, t10490, t10494, t10498, t10505, t10509, t10513)
}
