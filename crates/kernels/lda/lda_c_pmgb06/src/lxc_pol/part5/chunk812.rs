//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 812/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk812<F: Float>(t3172: F, t7295: F, t1462: F, t493: F, t1988: F, t2465: F, t1439: F, t7284: F, t442: F, t439: F, t1465: F, t496: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t7659 = t3172 * t7295;
    let t7660 = t1462 * t7659;
    let t7662 = F::new(2.0) / F::new(9.0) * t493 * t7660;
    let t7663 = t1988 * t2465;
    let t7665 = t493 * t7663 / F::new(15.0);
    let t7666 = t1439 * t7284;
    let t7667 = t442 * t7666;
    let t7669 = F::new(2.0) / F::new(15.0) * t439 * t7667;
    let t7670 = t1465 * t7295;
    let t7671 = t496 * t7670;
    (t7659, t7660, t7662, t7663, t7665, t7666, t7667, t7669, t7670, t7671)
}
