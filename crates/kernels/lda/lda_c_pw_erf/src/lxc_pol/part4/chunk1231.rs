//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1231/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1231<F: Float>(t1446: F, t6493: F, t1519: F, t2443: F, t6465: F, t1318: F, t2065: F, t5269: F, t5270: F, t2397: F, t3709: F, t6461: F, t2137: F, t6851: F, t18257: F, t18259: F, t18261: F, t18263: F, t18267: F, t18269: F, t18271: F, t18273: F, t18277: F) -> (F, F, F, F, F, F, F, F) {
    let t18279 = 16.0 / 15.0 * t1446 * t6493;
    let t18280 = t2443 * t1519;
    let t18281 = 4.0 / 135.0 * t18280;
    let t18283 = 32.0 / 45.0 * t1446 * t6465;
    let t18287 = 32.0 / 15.0 * t1318 * t5269 * t5270 * t2065;
    let t18289 = 8.0 / 45.0 * t3709 * t2397;
    let t18291 = 16.0 / 45.0 * t1446 * t6461;
    let t18292 = t6851 * t2137;
    let t18293 = 32.0 / 45.0 * t18292;
    let t18294 = t18257 + t18259 + t18261 + t18263 - t18267 + t18269 + t18271 + t18273 + t18277 + t18279 - t18281 - t18283 + t18287 - t18289 - t18291 + t18293;
    (t18279, t18281, t18283, t18287, t18289, t18291, t18293, t18294)
}
