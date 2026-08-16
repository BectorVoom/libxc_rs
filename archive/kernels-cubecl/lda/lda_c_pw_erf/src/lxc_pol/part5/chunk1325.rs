//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1325/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1325<F: Float>(t21500: F, t21505: F, t21509: F, t21513: F, t21515: F, t21519: F, t21523: F, t21525: F, t21527: F, t21530: F, t21535: F, t21540: F, t21542: F) -> F {
    let t23250 = -t21500 - t21505 + t21509 + t21513 - t21515 - t21519 - t21523 + t21525 - t21527 + t21530 + t21535 - t21540 + t21542;
    t23250
}
