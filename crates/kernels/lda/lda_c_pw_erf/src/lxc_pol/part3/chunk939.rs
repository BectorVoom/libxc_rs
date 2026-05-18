//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 939/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk939<F: Float>(t1449: F, t3887: F, t519: F, t4009: F, t518: F, t3993: F, t4035: F, t1450: F, t3745: F, t3709: F, t4048: F, t9: F) -> (F, F, F, F, F, F, F) {
    let t10500 = t519 * t1449 * t3887;
    let t10502 = t4009 * t518;
    let t10505 = t3993 * t518;
    let t10508 = t4035 * t518;
    let t10515 = t3745 * t1450;
    let t10517 = t3709 * t1450;
    let t10527 = t9 * t4048;
    (t10500, t10502, t10505, t10508, t10515, t10517, t10527)
}
