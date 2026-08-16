//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 1052/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk1052<F: Float>(t47790: F, t47791: F, t43361: F, t43364: F, t43368: F, t43371: F, t43374: F, t43378: F, t43384: F, t43385: F, t43386: F, t43389: F, t43398: F, t43400: F, t43404: F, t43408: F, t47126: F, t47127: F, t47133: F, t47137: F, t47140: F) -> (F, F, F) {
    let t51074 = F::cast_from(2.0_f64) * t47790;
    let t51075 = F::cast_from(2.0_f64) * t47791;
    let t51082 = t43361 - t43364 - t43368 - t47126 - t43371 - t43374 - t47127 - t43378 + t43384 - t43385 - F::cast_from(0.76685851907841499352e0_f64) * t43386 + F::cast_from(0.72851559312449424384e1_f64) * t43389 + t43398 - F::cast_from(0.1533717038156829987e1_f64) * t43400 - t43404 + t43408 + F::cast_from(0.30674340763136599742e1_f64) * t47133 - F::cast_from(0.51123901271894332902e1_f64) * t47137 + F::cast_from(0.51123901271894332901e0_f64) * t47140;
    (t51074, t51075, t51082)
}
