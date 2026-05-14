//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 843/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk843<F: Float>(t5047: F, t5071: F, t5952: F, t5965: F, t5966: F, t5971: F, t5974: F, t9628: F, t9746: F, t9753: F, t9756: F, t9922: F, t9925: F, t9929: F, t9933: F, t9936: F, t9943: F) -> (F,) {
    let t10082 = -t5966 + t5971 + t5952 + t5965 + 0.15282509383508946 * t5047 - t5974 + 0.05094169794502982 * t5071 + 1.532302805120685 * t9922 - 1.532302805120685 * t9925 - 1.532302805120685 * t9929 + 2.2984542076810275 * t9933 - 1.532302805120685 * t9936 + 0.15282509383508946 * t9746 + 0.05094169794502982 * t9753 + 0.15282509383508946 * t9756 + 0.30565018767017893 * t9628 - 0.510767601706895 * t9943;
    (t10082,)
}
