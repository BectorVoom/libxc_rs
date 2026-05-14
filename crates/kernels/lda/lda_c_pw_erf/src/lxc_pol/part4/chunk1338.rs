//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1338/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1338<F: Float>(t18520: F, t18522: F, t18524: F, t18528: F, t18530: F, t18532: F, t18534: F, t18538: F, t18540: F, t18542: F, t18544: F, t18546: F, t18550: F, t18552: F, t18557: F, t18561: F, t18563: F) -> (F,) {
    let t19323 = -t18520 - t18522 - t18524 - t18528 + t18530 - t18532 - t18534 + t18538 - t18540 - t18542 - t18544 + t18546 + t18550 - t18552 - t18557 + t18561 + t18563;
    (t19323,)
}
