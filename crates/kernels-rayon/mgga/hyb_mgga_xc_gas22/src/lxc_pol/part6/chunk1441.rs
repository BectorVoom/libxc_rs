//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1441/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1441(t1129: f64, t11536: f64, t11544: f64, t26552: f64, t26564: f64, t26579: f64, t31225: f64, t31229: f64, t31304: f64, t31310: f64, t31311: f64, t31317: f64, t31322: f64, t31330: f64, t9632: f64, t9642: f64, t9667: f64, t9765: f64, t9769: f64, t9773: f64) -> f64 {
    let t31337 = 504.0_f64 * t9773 * t31304 + 24.0_f64 * t9765 * t31304 + 10000.0_f64 / 81.0_f64 * t31310 * t31311 - 360.0_f64 * t9769 * t11544 * t1129 + 504.0_f64 * t9773 * t31317 + 24.0_f64 * t9765 * t31317 - 96.0_f64 * t26552 * t31322 - 1440.0_f64 * t26579 * t11536 * t1129 - 4032.0_f64 * t26564 * t31322 + 1408.0_f64 / 81.0_f64 * t9642 * t31330 - 6400.0_f64 / 81.0_f64 * t9632 * t31229 - 1408.0_f64 / 243.0_f64 * t9667 * t31225;
    t31337
}
