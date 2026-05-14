//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 932/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk932<F: Float>(t123: F, t290: F, t317: F, t8101: F, t113: F, t301: F, t3951: F, t83: F, t1166: F, t247: F, t3993: F, t413: F, t2789: F, t718: F, t1135: F, t1183: F) -> (F, F, F, F, F, F) {
    let t10599 = 5.240451065072324 * t123 * t8101 * t290 * t317;
    let t10603 = 1.0943113336969376e-06 * t3951 * t83 * t113 * t301;
    let t10606 = t247 * t1166 * t113 * t301;
    let t10609 = t3993 * t413 * t301;
    let t10614 = 0.0011622696607154768 * t718 * t2789 * t301;
    let t10617 = 0.008135887625008338 * t1135 * t1183 * t301;
    (t10599, t10603, t10606, t10609, t10614, t10617)
}
