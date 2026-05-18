//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 1003/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk1003<F: Float>(t365: F, t9739: F, t5047: F, t5071: F, t5426: F, t5439: F, t5440: F, t5445: F, t5448: F, t9628: F, t9746: F, t9753: F, t9756: F, t9922: F, t9925: F, t9929: F, t9933: F, t9936: F, t9943: F) -> (F, F) {
    let t10854 = t365 * t9739;
    let t10869 = -t5440 + t5445 + t5426 + t5439 + F::new(1.2466946262544771) * t5047 - t5448 + F::new(0.41556487541815906) * t5071 + F::new(12.5) * t9922 - F::new(12.5) * t9925 - F::new(12.5) * t9929 + F::new(18.75) * t9933 - F::new(12.5) * t9936 + F::new(1.2466946262544771) * t9746 + F::new(0.41556487541815906) * t9753 + F::new(1.2466946262544771) * t9756 + F::new(2.4933892525089543) * t9628 - F::new(4.166666666666667) * t9943;
    (t10854, t10869)
}
