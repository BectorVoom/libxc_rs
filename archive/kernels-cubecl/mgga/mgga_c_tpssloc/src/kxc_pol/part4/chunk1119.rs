//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 1119/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk1119<F: Float>(t1213: F, t18375: F, t1216: F, t5979: F, t3578: F, t5975: F, t11678: F, t11709: F, t11734: F, t1227: F, t15438: F, t15569: F, t18342: F, t18346: F, t18357: F, t18360: F, t18364: F, t18368: F, t18372: F, t3490: F, t3577: F, t4954: F, t4984: F, t5014: F, t5019: F, t6203: F, t6227: F, t6232: F) -> F {
    let t18376 = t1213 * t18375;
    let t18382 = t5979 * t1216;
    let t18383 = t3578 * t18382;
    let t18386 = t5975 * t1216;
    let t18387 = t3578 * t18386;
    let t18390 = -t15438 * t4984 / F::cast_from(1536.0_f64) + F::cast_from(5.0_f64) / F::cast_from(6912.0_f64) * t1227 * t18342 + F::cast_from(5.0_f64) / F::cast_from(2304.0_f64) * t1227 * t18346 + t11709 * t6227 / F::cast_from(1536.0_f64) - t11734 * t6232 / F::cast_from(3072.0_f64) - t5019 * t5014 / F::cast_from(288.0_f64) + t18357 / F::cast_from(2304.0_f64) - t3577 * t18360 / F::cast_from(2304.0_f64) + F::cast_from(5.0_f64) / F::cast_from(13824.0_f64) * t3577 * t18364 - t11678 * t18368 / F::cast_from(2304.0_f64) - t18372 / F::cast_from(3456.0_f64) + t18376 / F::cast_from(4608.0_f64) + F::cast_from(5.0_f64) / F::cast_from(13824.0_f64) * t3490 * t6203 + t15569 * t4954 / F::cast_from(432.0_f64) - t3577 * t18383 / F::cast_from(4608.0_f64) - t3577 * t18387 / F::cast_from(2304.0_f64);
    t18390
}
