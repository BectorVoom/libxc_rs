//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 925/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk925<F: Float>(t1223: F, t2488: F, t5043: F, t5047: F, t5056: F, t5071: F, t6060: F, t6062: F, t6068: F, t9623: F, t9628: F, t9631: F, t9635: F, t9742: F, t9746: F, t9750: F, t9753: F, t9756: F) -> (F, F) {
    let t9777 = t2488 * t1223;
    let t9796 = t6060 - F::new(1.5625) * t5043 + t6062 + F::new(1.5625) * t5047 - F::new(1.5625) * t9623 + F::new(3.125) * t9628 - F::cast_from(0.5208333333333334_f64) * t9631 - F::new(1.5625) * t9635 - F::new(1.5625) * t9742 - F::cast_from(0.5208333333333334_f64) * t5056 - t6068 + F::cast_from(0.5208333333333334_f64) * t5071 + F::new(1.5625) * t9746 - F::new(1.5625) * t9750 + F::cast_from(0.5208333333333334_f64) * t9753 + F::new(1.5625) * t9756;
    (t9777, t9796)
}
