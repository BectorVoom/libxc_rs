//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 920/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk920(t1231: f64, t1861: f64, t26: f64, t1819: f64, t2998: f64, t555: f64, t1808: f64, t2997: f64, t1181: f64, t1874: f64, t1877: f64, t1804: f64, t1807: f64, t19: f64, t558: f64, t6201: f64, t6204: f64, t6207: f64, t6216: f64, t8183: f64, t8187: f64, t8189: f64, t8193: f64, t8199: f64, t8201: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8204 = t1231 * t1861;
    let t8205 = t26 * t8204;
    let t8210 = t555 * t1819 * t2998 / 96.0_f64;
    let t8211 = t2997 * t1808;
    let t8216 = t1181 * t1874 / 32.0_f64;
    let t8218 = t1181 * t1877 / 32.0_f64;
    let t8219 = -t6201 / 96.0_f64 - t6204 / 96.0_f64 - t6207 / 192.0_f64 - t6216 / 144.0_f64 - t8183 + 7.0_f64 / 96.0_f64 * t8187 - t555 * t558 * t8189 / 64.0_f64 - t555 * t558 * t8193 / 32.0_f64 - t8199 - 3.0_f64 / 32.0_f64 * t19 * t8201 - 3.0_f64 / 64.0_f64 * t19 * t8205 - t8210 - t1804 * t1807 * t8211 / 48.0_f64 - t8216 - t8218;
    (t8204, t8205, t8210, t8211, t8216, t8218, t8219)
}
