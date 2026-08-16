//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1236/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1236(t1125: f64, t763: f64, t133: f64, t1844: f64, t474: f64, t156: f64, t5549: f64, t1823: f64, t343: f64, t1829: f64, t11411: f64, t11419: f64, t11422: f64, t11437: f64, t11445: f64, t11448: f64, t1558: f64, t1563: f64, t1820: f64, t1826: f64, t2954: f64, t2961: f64, t2967: f64, t2973: f64, t3234: f64, t3243: f64, t34: f64, t348: f64, t352: f64, t39: f64, t406: f64, t408: f64, t462: f64, t5524: f64, t5527: f64, t5536: f64, t5539: f64, t739: f64, t743: f64, t8949: f64, t8962: f64, t9456: f64, t9481: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14581 = t1125 * t763;
    let t14582 = t133 * t14581;
    let t14584 = t474 * t1844;
    let t14585 = t133 * t14584;
    let t14587 = t156 * t5549;
    let t14588 = t133 * t14587;
    let t14616 = 8.0_f64 * t1823 * t343;
    let t14631 = 8.0_f64 * t1829 * t343;
    let t14632 = -t1820 * t2961 / 9.0_f64 - 4.0_f64 * t406 * t39 - t1826 * t2973 / 9.0_f64 + 4.0_f64 * t408 * t39 + 2.0_f64 * t5527 * t11419 - 2.0_f64 * t5539 * t11445 - 28.0_f64 / 81.0_f64 * t8949 * t739 * t2954 + 4.0_f64 / 9.0_f64 * t5524 * t9481 + 8.0_f64 / 9.0_f64 * t3234 * t34 * t11411 - 2.0_f64 / 3.0_f64 * t1558 * t462 * t348 - 2.0_f64 / 3.0_f64 * t5527 * t11422 + t14616 - 28.0_f64 / 81.0_f64 * t8962 * t743 * t2967 + 4.0_f64 / 9.0_f64 * t5536 * t9456 - 8.0_f64 / 9.0_f64 * t3243 * t34 * t11437 + 2.0_f64 / 3.0_f64 * t1563 * t462 * t352 + 2.0_f64 / 3.0_f64 * t5539 * t11448 - t14631;
    (t14581, t14582, t14584, t14585, t14587, t14588, t14632)
}
