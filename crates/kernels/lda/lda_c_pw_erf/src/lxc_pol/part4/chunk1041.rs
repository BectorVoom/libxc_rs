//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1041/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1041<F: Float>(t1844: F, t474: F, t133: F, t156: F, t5549: F, t1823: F, t343: F, t1829: F, t21: F, t411: F, t635: F, t1652: F, t763: F, t415: F, t5594: F, t1568: F, t4: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t14584 = t474 * t1844;
    let t14585 = t133 * t14584;
    let t14587 = t156 * t5549;
    let t14588 = t133 * t14587;
    let t14616 = 8.0 * t1823 * t343;
    let t14631 = 8.0 * t1829 * t343;
    let t14639 = t21 * t635 * t411;
    let t14640 = t1652 * t763 * t14639;
    let t14643 = t415 * t1844 * t5594;
    let t14646 = t4 * t156 * t1568;
    (t14584, t14585, t14587, t14588, t14616, t14631, t14639, t14640, t14643, t14646)
}
