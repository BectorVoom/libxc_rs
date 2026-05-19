//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 166/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk166<F: Float>(t429: F, t537: F, t435: F, t441: F, t305: F) -> (F, F, F, F) {
    let t538 = t537 * t429;
    let t543 = F::new(3.125) * t435 + F::cast_from(1.2466946262544771_f64) * t441 + F::cast_from(0.146484375_f64);
    let t544 = F::ln(t543);
    let t545 = t544 * t305;
    (t538, t543, t544, t545)
}
