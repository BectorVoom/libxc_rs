//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 983/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk983<F: Float>(t43: F, t11401: F, t3160: F, t749: F, t3166: F, t462: F, t940: F, t348: F, t39: F, t945: F, t1784: F, t343: F, t1781: F, t2953: F, t2954: F, t2961: F, t34: F, t4352: F, t4355: F, t47: F, t739: F, t8315: F, t939: F, t9481: F, zeta_threshold: F) -> (F, F, F, F, F, F, F) {
    let t44 = t43 <= zeta_threshold;
    let t11402 = F::new(24.0) * t11401;
    let t11403 = t3160 * t749;
    let t11404 = F::new(240.0) * t11403;
    let t11405 = t3166 * t749;
    let t11406 = F::new(120.0) * t11405;
    let t11411 = t462 * t940;
    let t11419 = t39 * t348;
    let t11422 = t462 * t945;
    let t11430 = F::new(32.0) * t1784 * t343;
    let t11432 = piecewise3::<f64>(t44, F::new(0.0), F::new(40.0) / F::new(81.0) * t8315 * t739 * t2954 - F::new(16.0) / F::new(9.0) * t2953 * t34 * t11411 - F::new(8.0) / F::new(9.0) * t4352 * t9481 + F::new(8.0) / F::new(3.0) * t939 * t462 * t348 - F::new(8.0) * t4355 * t11419 + F::new(8.0) / F::new(3.0) * t4355 * t11422 + F::new(4.0) / F::new(9.0) * t1781 * t2961 - F::new(16.0) * t47 * t39 + t11430);
    (t11402, t11404, t11406, t11411, t11419, t11422, t11432)
}
