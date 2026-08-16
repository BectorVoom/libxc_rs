//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1120/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1120(t76517: f64, t1540: f64, t3292: f64, t70556: f64, t70574: f64, t70578: f64, t76539: f64, t76542: f64, t78567: f64, t78571: f64, t78574: f64, t78575: f64, t78576: f64, t78577: f64, t78578: f64, t78582: f64, t78585: f64, t78588: f64) -> f64 {
    let t80530 = 0.16566831523319392754e-1_f64 * t76517;
    let t80534 = t78567 + t78571 - t80530 + t78574 - t78575 - t78576 + t78577 - t78578 + 0.40878380883436523435e-5_f64 * t70556 + t70574 + t70578 - 0.19957069503106347607e-1_f64 * t1540 * t3292 + t78582 + t76539 - t76542 - t78585 - t78588;
    t80534
}
