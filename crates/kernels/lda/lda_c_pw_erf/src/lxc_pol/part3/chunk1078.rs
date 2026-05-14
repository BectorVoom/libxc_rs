//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1078/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1078<F: Float>(t3296: F, t9: F, t155: F, t1697: F, t1870: F, t5652: F, t5515: F, t925: F, t2061: F, t5518: F, t325: F, t415: F, t5568: F, t1652: F, t1833: F, t933: F) -> (F, F, F, F, F, F) {
    let t14674 = t9 * t3296;
    let t14679 = t155 * t1697;
    let t14681 = t1870 * t14679 * t5652;
    let t14683 = t5515 * t925;
    let t14684 = 1.9486833333333333 * t14683;
    let t14685 = t5518 * t2061;
    let t14686 = 1.2991222222222223 * t14685;
    let t14688 = t415 * t5568 * t325;
    let t14689 = 1.4615125 * t14688;
    let t14691 = t1652 * t1833 * t933;
    (t14674, t14681, t14684, t14686, t14689, t14691)
}
