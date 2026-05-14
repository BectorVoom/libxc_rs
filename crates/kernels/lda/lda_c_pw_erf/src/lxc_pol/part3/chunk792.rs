//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 792/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk792<F: Float>(t260: F, t34: F, t343: F, t262: F, t3154: F, t344: F, t339: F, t311: F, t1062: F, t22: F, t19: F, t301: F, t305: F, t732: F, t1022: F, t2986: F) -> (F, F, F, F, F, F, F, F) {
    let t8315 = 1.0 / t260;
    let t8327 = t34 * t343;
    let t8334 = 1.0 / t262;
    let t8356 = 16.0 * t344 * t3154;
    let t8357 = t339 * t3154;
    let t8359 = t311 * t311;
    let t8363 = 1.0 / t22 / t1062;
    let t8368 = 0.3407285805772476 * t305 / t8359 * t8363 * t301 * t19 * t732;
    let t8370 = t2986 * t1022;
    (t8315, t8327, t8334, t8356, t8357, t8363, t8368, t8370)
}
