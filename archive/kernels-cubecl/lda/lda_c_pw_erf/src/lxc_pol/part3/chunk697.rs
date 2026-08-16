//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 697/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk697<F: Float>(t1752: F, t4299: F, t1746: F, t1759: F, t1742: F, t19: F, t729: F, t734: F, t2953: F, t739: F, t34: F, t939: F) -> (F, F, F, F, F, F) {
    let t4300 = t1752 * t4299;
    let t4304 = t1759 * t1746;
    let t4307 = t1742 * t729 * t19;
    let t4308 = t4307 * t734;
    let t4352 = t2953 * t739;
    let t4355 = t939 * t34;
    (t4300, t4304, t4307, t4308, t4352, t4355)
}
