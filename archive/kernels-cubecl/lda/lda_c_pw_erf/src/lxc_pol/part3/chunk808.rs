//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 808/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk808<F: Float>(t426: F, t5521: F, t3234: F, t739: F, t1558: F, t34: F, t3243: F, t743: F, t1563: F, t1820: F, t1823: F, t1826: F, t1829: F, t39: F, t406: F, t408: F, t4356: F, t4371: F, t462: F, t940: F, t945: F, t951: F, t954: F) -> (F, F, F, F, F, F) {
    let t5523 = t426 * t5521 / F::cast_from(3.0_f64);
    let t5524 = t3234 * t739;
    let t5527 = t1558 * t34;
    let t5536 = t3243 * t743;
    let t5539 = t1563 * t34;
    let t5548 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t5524 * t940 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t5527 * t4356 - t1820 * t945 / F::cast_from(9.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t406 * t462 - F::cast_from(2.0_f64) * t1823 * t39 + F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t5536 * t951 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t5539 * t4371 - t1826 * t954 / F::cast_from(9.0_f64) - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t408 * t462 + F::cast_from(2.0_f64) * t1829 * t39;
    (t5523, t5524, t5527, t5536, t5539, t5548)
}
