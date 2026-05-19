//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 462/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk462<F: Float>(t1660: F, t415: F, t760: F, t325: F, t102: F, t411: F, t763: F, t1558: F, t739: F, t34: F, t406: F, t1563: F, t743: F) -> (F, F, F, F, F, F, F) {
    let t1813 = F::cast_from(0.48717083333333333_f64) * t1660;
    let t1814 = t415 * t760;
    let t1815 = t1814 * t325;
    let t1816 = F::cast_from(0.48717083333333333_f64) * t1815;
    let t1819 = F::new(5.84605) * t102 * t763 * t411;
    let t1820 = t1558 * t739;
    let t1823 = t406 * t34;
    let t1826 = t1563 * t743;
    (t1813, t1814, t1816, t1819, t1820, t1823, t1826)
}
