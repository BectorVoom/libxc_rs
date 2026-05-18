//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1236/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1236<F: Float>(t1125: F, t763: F, t133: F, t1844: F, t474: F, t156: F, t5549: F, t1823: F, t343: F, t1829: F, t11411: F, t11419: F, t11422: F, t11437: F, t11445: F, t11448: F, t1558: F, t1563: F, t1820: F, t1826: F, t2954: F, t2961: F, t2967: F, t2973: F, t3234: F, t3243: F, t34: F, t348: F, t352: F, t39: F, t406: F, t408: F, t462: F, t5524: F, t5527: F, t5536: F, t5539: F, t739: F, t743: F, t8949: F, t8962: F, t9456: F, t9481: F) -> (F, F, F, F, F, F, F) {
    let t14581 = t1125 * t763;
    let t14582 = t133 * t14581;
    let t14584 = t474 * t1844;
    let t14585 = t133 * t14584;
    let t14587 = t156 * t5549;
    let t14588 = t133 * t14587;
    let t14616 = F::new(8.0) * t1823 * t343;
    let t14631 = F::new(8.0) * t1829 * t343;
    let t14632 = -t1820 * t2961 / F::new(9.0) - F::new(4.0) * t406 * t39 - t1826 * t2973 / F::new(9.0) + F::new(4.0) * t408 * t39 + F::new(2.0) * t5527 * t11419 - F::new(2.0) * t5539 * t11445 - F::new(28.0) / F::new(81.0) * t8949 * t739 * t2954 + F::new(4.0) / F::new(9.0) * t5524 * t9481 + F::new(8.0) / F::new(9.0) * t3234 * t34 * t11411 - F::new(2.0) / F::new(3.0) * t1558 * t462 * t348 - F::new(2.0) / F::new(3.0) * t5527 * t11422 + t14616 - F::new(28.0) / F::new(81.0) * t8962 * t743 * t2967 + F::new(4.0) / F::new(9.0) * t5536 * t9456 - F::new(8.0) / F::new(9.0) * t3243 * t34 * t11437 + F::new(2.0) / F::new(3.0) * t1563 * t462 * t352 + F::new(2.0) / F::new(3.0) * t5539 * t11448 - t14631;
    (t14581, t14582, t14584, t14585, t14587, t14588, t14632)
}
