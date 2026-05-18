//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 310/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk310<F: Float>(t418: F, t76: F, t481: F, t5: F, t83: F, t6: F, t995: F, t93: F, t414: F, t491: F, t1141: F, t1146: F, t1147: F, t1174: F, t1249: F, t392: F, t402: F, t405: F, t421: F, t70: F, t73: F, t99: F) -> (F, F, F, F, F) {
    let t1254 = t76 * t418;
    let t1256 = t5 * t481;
    let t1257 = t83 * t1256;
    let t1260 = t6 * t995;
    let t1261 = t93 * t1260;
    let t1263 = -F::new(0.11955719325063177623e-1) * t414 + F::new(0.40985e-2) * t1254 - F::new(0.10566666666666666667e-2) * t1257 + F::new(0.3884654180847230157e-4) * t491 - F::new(0.420109375e-5) * t1261;
    let t1265 = F::new(0.23426533963880895498e-2) * t414 * t70 + F::new(0.46853067927761790996e-2) * t1141 * t402 + F::new(0.70279601891642686494e-2) * t1146 * t1147 - F::new(0.23426533963880895498e-2) * t392 * t1174 - t1249 * t99 - F::new(2.0) * t405 * t421 - t73 * t1263;
    (t1254, t1257, t1261, t1263, t1265)
}
