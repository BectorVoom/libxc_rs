//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 666/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk666<F: Float>(t1468: F, t354: F, t1284: F, t5012: F, t323: F, t359: F, t5031: F, t402: F, t1470: F, t4943: F, t1476: F, t5084: F) -> (F, F, F, F, F, F) {
    let t6130 = t354 * t1468;
    let t6131 = t6130 * t1284;
    let t6133 = F::new(3.7610742193750633) * t6131 * t5012;
    let t6134 = t323 * t1468;
    let t6135 = t6134 * t1284;
    let t6137 = F::new(7.5221484387501265) * t6135 * t5012;
    let t6138 = t359 * t5031;
    let t6149 = t402 * t1468;
    let t6150 = t6149 * t1284;
    let t6152 = F::new(4.855032390388656) * t6150 * t5012;
    let t6154 = F::new(9.477567664245134) * t1470 * t4943;
    let t6155 = t1476 * t5084;
    (t6133, t6137, t6138, t6152, t6154, t6155)
}
