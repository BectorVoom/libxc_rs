//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 797/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk797<F: Float>(t1440: F, t5359: F, t519: F, t2192: F, t3899: F, t1318: F, t1381: F, t2191: F, t1466: F, t5329: F, t5331: F, t5333: F, t5336: F, t5338: F, t5341: F, t5344: F, t5346: F, t5348: F, t5350: F, t5352: F, t5354: F, t5358: F) -> (F, F, F, F, F, F, F, F) {
    let t5360 = t1440 * t5359;
    let t5362 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t519 * t5360;
    let t5363 = t3899 * t2192;
    let t5365 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t1318 * t5363;
    let t5366 = t2191 * t1381;
    let t5367 = t1466 * t5366;
    let t5369 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t1318 * t5367;
    let t5370 = t5329 - t5331 + t5333 + t5336 + t5338 - t5341 - t5344 + t5346 + t5348 + t5350 + t5352 - t5354 - t5358 + t5362 - t5365 - t5369;
    (t5360, t5362, t5363, t5365, t5366, t5367, t5369, t5370)
}
