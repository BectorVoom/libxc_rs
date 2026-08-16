//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 960/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk960(t43400: f64, t33308: f64, t9805: f64, t9806: f64, t15499: f64, t28640: f64, t3487: f64, t40966: f64, t13101: f64, t1991: f64, t40942: f64, t40946: f64, t43361: f64, t43364: f64, t43368: f64, t43371: f64, t43374: f64, t43378: f64, t43384: f64, t43385: f64, t43387: f64, t43390: f64, t43393: f64, t43398: f64, t590: f64) -> f64 {
    let t43401 = 0.15337170381568299871e1_f64 * t43400;
    let t43403 = t9805 * t33308 * t9806;
    let t43404 = 0.10352590007558602413e2_f64 * t43403;
    let t43407 = t28640 * t15499 * t3487 * t9806;
    let t43408 = 0.46011511144704899612e1_f64 * t43407;
    let t43409 = 0.11502877786176224903e1_f64 * t40966;
    let t43410 = t43361 - t43364 - t43368 - 0.1533717038156829987e1_f64 * t40942 - t43371 - t43374 - 0.38342925953920749676e0_f64 * t40946 - t43378 + 0.51123901271894332902e0_f64 * t1991 * t13101 * t590 + t43384 - t43385 - t43387 + t43390 + 0.30674340763136599742e1_f64 * t43393 + t43398 - t43401 - t43404 + t43408 + t43409;
    t43410
}
