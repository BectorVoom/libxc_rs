//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 1052/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk1052(t47790: f64, t47791: f64, t43361: f64, t43364: f64, t43368: f64, t43371: f64, t43374: f64, t43378: f64, t43384: f64, t43385: f64, t43386: f64, t43389: f64, t43398: f64, t43400: f64, t43404: f64, t43408: f64, t47126: f64, t47127: f64, t47133: f64, t47137: f64, t47140: f64) -> (f64, f64, f64) {
    let t51074 = 2.0_f64 * t47790;
    let t51075 = 2.0_f64 * t47791;
    let t51082 = t43361 - t43364 - t43368 - t47126 - t43371 - t43374 - t47127 - t43378 + t43384 - t43385 - 0.76685851907841499352e0_f64 * t43386 + 0.72851559312449424384e1_f64 * t43389 + t43398 - 0.1533717038156829987e1_f64 * t43400 - t43404 + t43408 + 0.30674340763136599742e1_f64 * t47133 - 0.51123901271894332902e1_f64 * t47137 + 0.51123901271894332901e0_f64 * t47140;
    (t51074, t51075, t51082)
}
