//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 929/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk929(t178: f64, t2104: f64, t1270: f64, t173: f64, t180: f64, t3227: f64, t3232: f64, t3245: f64, t3246: f64, t3252: f64, t3255: f64, t3258: f64, t6355: f64, t747: f64, t751: f64, t8354: f64, t8373: f64, t8395: f64, t8396: f64, t8399: f64, t8402: f64, t8410: f64, t8415: f64, t8418: f64) -> (f64, f64) {
    let t8423 = t178 * t2104;
    let t8431 = 15.0_f64 / 2.0_f64 * t8395 * t8396 - 4.0_f64 * t8399 * t3246 - 5.0_f64 / 2.0_f64 * t8402 * t8396 - 2.0_f64 * t3245 * t6355 + t747 * t8354 * t180 / 2.0_f64 + t8410 * t3246 / 2.0_f64 + t3252 * t6355 / 4.0_f64 + t8415 * t8396 / 8.0_f64 - 4.0_f64 * t8418 * t1270 - 8.0_f64 * t3255 * t3227 - t8423 * t3232 - 2.0_f64 * t3258 * t8373 - 4.0_f64 * t751 * t8354 - t173 * t8354 * t180;
    (t8423, t8431)
}
