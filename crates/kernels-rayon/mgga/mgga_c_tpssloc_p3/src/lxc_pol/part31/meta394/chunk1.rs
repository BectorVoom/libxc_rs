//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1422/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1422(t1213: f64, t18375: f64, t1216: f64, t5979: f64, t3578: f64, t5975: f64, t11678: f64, t11709: f64, t11734: f64, t1227: f64, t15438: f64, t15569: f64, t18342: f64, t18346: f64, t18357: f64, t18360: f64, t18364: f64, t18368: f64, t18372: f64, t3490: f64, t3577: f64, t4954: f64, t4984: f64, t5014: f64, t5019: f64, t6203: f64, t6227: f64, t6232: f64) -> f64 {
    let t18376 = t1213 * t18375;
    let t18382 = t5979 * t1216;
    let t18383 = t3578 * t18382;
    let t18386 = t5975 * t1216;
    let t18387 = t3578 * t18386;
    let t18390 = -t15438 * t4984 / 1536.0_f64 + 5.0_f64 / 6912.0_f64 * t1227 * t18342 + 5.0_f64 / 2304.0_f64 * t1227 * t18346 + t11709 * t6227 / 1536.0_f64 - t11734 * t6232 / 3072.0_f64 - t5019 * t5014 / 288.0_f64 + t18357 / 2304.0_f64 - t3577 * t18360 / 2304.0_f64 + 5.0_f64 / 13824.0_f64 * t3577 * t18364 - t11678 * t18368 / 2304.0_f64 - t18372 / 3456.0_f64 + t18376 / 4608.0_f64 + 5.0_f64 / 13824.0_f64 * t3490 * t6203 + t15569 * t4954 / 432.0_f64 - t3577 * t18383 / 4608.0_f64 - t3577 * t18387 / 2304.0_f64;
    t18390
}
