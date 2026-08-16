//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1170/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1170(t4641: f64, t6190: f64, t1074: f64, t2570: f64, t36: f64, t453: f64, t350: f64, t6161: f64, t6166: f64, t6176: f64, t6179: f64, t6182: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t15393 = t4641 * t6190;
    let t15395 = t2570 * t1074;
    let t15397 = t36 * t453 * t15395;
    let t15399 = t350 * t6161;
    let t15401 = t350 * t6166;
    let t15403 = t350 * t6176;
    let t15405 = t350 * t6179;
    let t15407 = t4641 * t6182;
    (t15393, t15395, t15397, t15399, t15401, t15403, t15405, t15407)
}
