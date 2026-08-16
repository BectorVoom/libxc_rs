//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 797/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk797(t1440: f64, t5359: f64, t519: f64, t2192: f64, t3899: f64, t1318: f64, t1381: f64, t2191: f64, t1466: f64, t5329: f64, t5331: f64, t5333: f64, t5336: f64, t5338: f64, t5341: f64, t5344: f64, t5346: f64, t5348: f64, t5350: f64, t5352: f64, t5354: f64, t5358: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5360 = t1440 * t5359;
    let t5362 = 4.0_f64 / 15.0_f64 * t519 * t5360;
    let t5363 = t3899 * t2192;
    let t5365 = 16.0_f64 / 45.0_f64 * t1318 * t5363;
    let t5366 = t2191 * t1381;
    let t5367 = t1466 * t5366;
    let t5369 = 4.0_f64 / 15.0_f64 * t1318 * t5367;
    let t5370 = t5329 - t5331 + t5333 + t5336 + t5338 - t5341 - t5344 + t5346 + t5348 + t5350 + t5352 - t5354 - t5358 + t5362 - t5365 - t5369;
    (t5360, t5362, t5363, t5365, t5366, t5367, t5369, t5370)
}
