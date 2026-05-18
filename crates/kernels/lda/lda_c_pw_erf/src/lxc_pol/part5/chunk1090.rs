//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1090/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1090<F: Float>(t1558: F, t1563: F, t17673: F, t1820: F, t1826: F, t19994: F, t19997: F, t20007: F, t20019: F, t20027: F, t348: F, t352: F, t406: F, t408: F, t5524: F, t5527: F, t5536: F, t5539: F, t5992: F, t6005: F, t6101: F, t6111: F, t7354: F, t7360: F, t7365: F, t7370: F, t8949: F, t8962: F, t943: F) -> F {
    let t20283 = -F::new(28.0) / F::new(81.0) * t8949 * t7354 * t348 + F::new(8.0) / F::new(9.0) * t6101 * t943 + F::new(4.0) / F::new(9.0) * t5524 * t19994 - F::new(2.0) / F::new(3.0) * t5527 * t19997 - t1820 * t5992 / F::new(3.0) - t1558 * t7360 * t348 / F::new(9.0) + t406 * t20007 / F::new(3.0) - F::new(28.0) / F::new(81.0) * t8962 * t7365 * t352 - F::new(8.0) / F::new(9.0) * t6111 * t943 + F::new(4.0) / F::new(9.0) * t5536 * t17673 + F::new(2.0) / F::new(3.0) * t5539 * t20019 - t1826 * t6005 / F::new(3.0) - t1563 * t7370 * t352 / F::new(9.0) + t408 * t20027 / F::new(3.0);
    t20283
}
