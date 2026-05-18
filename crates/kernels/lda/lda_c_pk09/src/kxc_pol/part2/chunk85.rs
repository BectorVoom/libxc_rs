//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 85/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk85<F: Float>(t43: F, t6: F, t2: F, t3: F, t1: F, t4: F) -> (F, F, F, F, F) {
    let t254 = F::new(1.1801314654631911) * t43;
    let t255 = F::new(1.4269304149842164) * t6;
    let t256 = t3 * t2;
    let t257 = t4 * t1;
    let t258 = F::new(1.0) / t257;
    (t254, t255, t256, t257, t258)
}
