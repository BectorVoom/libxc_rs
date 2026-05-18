//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 926/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk926<F: Float>(t1439: F, t9796: F, t1449: F, t5040: F, t5043: F, t5046: F, t5047: F, t5056: F, t5069: F, t5071: F, t9623: F, t9628: F, t9631: F, t9635: F, t9742: F, t9746: F, t9750: F, t9753: F, t9756: F) -> (F, F) {
    let t9797 = t1439 * t9796;
    let t9798 = t9797 * t1449;
    let t9814 = t5040 - F::new(2.0) * t5043 + t5046 + F::new(2.0) * t5047 - F::new(2.0) * t9623 + F::new(4.0) * t9628 - F::new(2.0) / F::new(3.0) * t9631 - F::new(2.0) * t9635 - F::new(2.0) * t9742 - F::new(2.0) / F::new(3.0) * t5056 - t5069 + F::new(2.0) / F::new(3.0) * t5071 + F::new(2.0) * t9746 - F::new(2.0) * t9750 + F::new(2.0) / F::new(3.0) * t9753 + F::new(2.0) * t9756;
    (t9798, t9814)
}
