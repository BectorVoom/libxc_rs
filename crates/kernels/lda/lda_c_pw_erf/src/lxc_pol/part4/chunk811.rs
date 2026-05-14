//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 811/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk811<F: Float>(t2253: F, t656: F, t2256: F, t1410: F, t851: F, t5333: F, t5336: F, t5338: F, t5341: F, t5344: F, t5346: F, t5348: F, t5350: F, t5352: F, t5354: F, t5358: F, t5362: F, t5365: F, t5369: F) -> (F, F, F, F) {
    let t5871 = 4.0 / 9.0 * t2253 * t656;
    let t5872 = t2256 * t656;
    let t5874 = t851 * t1410;
    let t5876 = t5333 + t5336 + t5871 + 4.0 / 9.0 * t5872 - 2.0 / 27.0 * t5874 + t5338 - t5341 - t5344 + t5346 + t5348 + t5350 + t5352 - t5354 - t5358 + t5362 - t5365 - t5369;
    (t5871, t5872, t5874, t5876)
}
