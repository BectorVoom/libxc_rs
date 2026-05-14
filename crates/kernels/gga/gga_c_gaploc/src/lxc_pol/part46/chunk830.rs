//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 830/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk830<F: Float>(t43107: F, t5241: F, t5640: F, t590: F, t11068: F, t2679: F, t9796: F, t33308: F, t9805: F, t9806: F, t15499: F, t28640: F, t3487: F, t40966: F, t13101: F, t1991: F, t40942: F, t40946: F, t43361: F, t43364: F, t43368: F, t43371: F, t43374: F, t43378: F, t43384: F, t43385: F, t43387: F, t43390: F, t43393: F) -> (F,) {
    let t43398 = 0.15337170381568299871e1 * t5640 * t5241 * t43107 * t590;
    let t43400 = t9796 * t11068 * t2679;
    let t43401 = 0.15337170381568299871e1 * t43400;
    let t43403 = t9805 * t33308 * t9806;
    let t43404 = 0.10352590007558602413e2 * t43403;
    let t43407 = t28640 * t15499 * t3487 * t9806;
    let t43408 = 0.46011511144704899612e1 * t43407;
    let t43409 = 0.11502877786176224903e1 * t40966;
    let t43410 = t43361 - t43364 - t43368 - 0.1533717038156829987e1 * t40942 - t43371 - t43374 - 0.38342925953920749676e0 * t40946 - t43378 + 0.51123901271894332902e0 * t1991 * t13101 * t590 + t43384 - t43385 - t43387 + t43390 + 0.30674340763136599742e1 * t43393 + t43398 - t43401 - t43404 + t43408 + t43409;
    (t43410,)
}
