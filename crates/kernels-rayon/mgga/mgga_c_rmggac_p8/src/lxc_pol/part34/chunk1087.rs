//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1087/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1087(t72147: f64, t70582: f64, t2211: f64, t41122: f64, t884: f64, t40940: f64, t70556: f64, t70573: f64, t70577: f64, t76517: f64, t76539: f64, t76542: f64, t78567: f64, t78571: f64, t78572: f64, t78574: f64, t78575: f64, t78576: f64, t78577: f64) -> f64 {
    let t78578 = 0.36366215538993788972e-1_f64 * t72147;
    let t78582 = 0.86737941314158990619e-4_f64 * t70582;
    let t78585 = 0.11974241701863808564e0_f64 * t884 * t2211 * t41122;
    let t78588 = 0.11974241701863808564e0_f64 * t884 * t2211 * t40940;
    let t78589 = t78567 + t78571 + t78572 - 0.16566831523319392755e-1_f64 * t76517 + t78574 - t78575 - t78576 + t78577 - t78578 + 0.40878380883436523436e-5_f64 * t70556 + 0.17347588262831798123e-4_f64 * t70573 + 0.17347588262831798123e-4_f64 * t70577 + t78582 + t76539 - t76542 - t78585 - t78588;
    t78589
}
