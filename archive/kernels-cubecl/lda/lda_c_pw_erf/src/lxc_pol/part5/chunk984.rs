//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 984/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk984<F: Float>(t14581: F, t426: F, t1849: F, t1953: F, t117: F, t174: F, t14654: F, t8896: F, t127: F, t3296: F, t5509: F, t925: F) -> (F, F, F, F, F, F) {
    let t14732 = t426 * t14581;
    let t14734 = t1849 * t1953;
    let t14777 = t117 * t174;
    let t14781 = t8896 * t14654;
    let t14783 = t127 * t3296;
    let t14795 = t5509 * t925;
    (t14732, t14734, t14777, t14781, t14783, t14795)
}
