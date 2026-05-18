//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1241/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1241<F: Float>(t14688: F, t1652: F, t1833: F, t933: F, t102: F, t120: F, t14632: F, t1870: F, t1872: F, t436: F, t473: F, t5639: F, t5643: F) -> (F, F, F, F, F) {
    let t14689 = F::new(1.4615125) * t14688;
    let t14691 = t1652 * t1833 * t933;
    let t14692 = F::new(0.9743416666666667) * t14691;
    let t14695 = F::new(2.923025) * t102 * t120 * t14632;
    let t14698 = t1870 * t473 * t436 * t1872;
    let t14701 = t1870 * t5639 * t5643;
    (t14689, t14692, t14695, t14698, t14701)
}
