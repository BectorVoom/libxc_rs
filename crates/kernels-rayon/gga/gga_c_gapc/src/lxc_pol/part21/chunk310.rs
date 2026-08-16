//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 310/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk310(t418: f64, t76: f64, t481: f64, t5: f64, t83: f64, t6: f64, t995: f64, t93: f64, t414: f64, t491: f64, t1141: f64, t1146: f64, t1147: f64, t1174: f64, t1249: f64, t392: f64, t402: f64, t405: f64, t421: f64, t70: f64, t73: f64, t99: f64) -> (f64, f64, f64, f64, f64) {
    let t1254 = t76 * t418;
    let t1256 = t5 * t481;
    let t1257 = t83 * t1256;
    let t1260 = t6 * t995;
    let t1261 = t93 * t1260;
    let t1263 = -0.11955719325063177623e-1_f64 * t414 + 0.40985e-2_f64 * t1254 - 0.10566666666666666667e-2_f64 * t1257 + 0.3884654180847230157e-4_f64 * t491 - 0.420109375e-5_f64 * t1261;
    let t1265 = 0.23426533963880895498e-2_f64 * t414 * t70 + 0.46853067927761790996e-2_f64 * t1141 * t402 + 0.70279601891642686494e-2_f64 * t1146 * t1147 - 0.23426533963880895498e-2_f64 * t392 * t1174 - t1249 * t99 - 2.0_f64 * t405 * t421 - t73 * t1263;
    (t1254, t1257, t1261, t1263, t1265)
}
