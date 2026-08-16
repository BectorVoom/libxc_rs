//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 689/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk689(t1758: f64, t5153: f64, t54: f64, t72: f64, t6329: f64, t55: f64, t1240: f64, t1729: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6510 = t1758 * t5153;
    let t6511 = t72 * t54;
    let t6515 = t6329 * t5153;
    let t6516 = t72 * t55;
    let t6517 = t1240 * t1729;
    let t6519 = t6515 * t6516 * t6517;
    (t6510, t6511, t6515, t6516, t6517, t6519)
}
