//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 942/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk942<F: Float>(t5047: F, t5071: F, t5516: F, t5529: F, t5530: F, t5535: F, t5538: F, t9628: F, t9746: F, t9753: F, t9756: F, t9922: F, t9925: F, t9929: F, t9933: F, t9936: F, t9943: F) -> F {
    let t9945 = -t5530 + t5535 + t5516 + t5529 + F::new(0.04525483399593904) * t5047 - t5538 + F::new(0.015084944665313014) * t5071 + F::new(0.4537481858318121) * t9922 - F::new(0.4537481858318121) * t9925 - F::new(0.4537481858318121) * t9929 + F::new(0.6806222787477182) * t9933 - F::new(0.4537481858318121) * t9936 + F::new(0.04525483399593904) * t9746 + F::new(0.015084944665313014) * t9753 + F::new(0.04525483399593904) * t9756 + F::new(0.09050966799187808) * t9628 - F::new(0.15124939527727072) * t9943;
    t9945
}
