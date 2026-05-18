//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 932/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk932<F: Float>(t10230: F, t176: F, t4588: F, t517: F, t1925: F, t3223: F, t1592: F, t1962: F, t1989: F, t1980: F, t883: F, t1710: F, t1959: F) -> (F, F, F, F, F, F, F) {
    let t12592 = t10230 * t176;
    let t12617 = t4588 * t517;
    let t12621 = t3223 * t1925;
    let t12622 = F::new(2.0) / F::new(135.0) * t12621;
    let t12633 = t1962 * t1592;
    let t12649 = t3223 * t1989;
    let t12650 = F::new(2.0) / F::new(135.0) * t12649;
    let t12657 = t883 * t1980;
    let t12661 = t1959 * t1710;
    (t12592, t12617, t12622, t12633, t12650, t12657, t12661)
}
