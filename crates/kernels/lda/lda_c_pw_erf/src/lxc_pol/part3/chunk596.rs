//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 596/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk596<F: Float>(t101: F, t3365: F, t153: F, t274: F, t2869: F, t1089: F, t474: F, t1125: F, t678: F, t1298: F, t1386: F, t1394: F, t511: F) -> (F, F, F, F, F, F, F) {
    let t3366 = t101 * t3365;
    let t3373 = F::cast_from(4.429070076315393_f64) * t153 * t2869 * t274;
    let t3375 = t153 * t474 * t1089;
    let t3378 = t153 * t1125 * t678;
    let t3380 = t1298 * t1386;
    let t3381 = F::new(16.0) / F::new(15.0) * t3380;
    let t3383 = F::new(4.0) / F::new(5.0) * t511 * t1394;
    (t3366, t3373, t3375, t3378, t3380, t3381, t3383)
}
