//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1066/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1066(t1: f64, t6145: f64, t1525: f64, t1830: f64, t1858: f64, t5961: f64, t36: f64, t2381: f64, t4654: f64, t332: f64, t7481: f64, t453: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t19754 = t6145 * t1;
    let t19756 = t1830 * t1525 * t19754;
    let t19758 = t1858 * t5961;
    let t19760 = t36 * t1525 * t19758;
    let t19762 = t4654 * t2381;
    let t19764 = t1830 * t1525 * t19762;
    let t19766 = t7481 * t332;
    let t19768 = t36 * t453 * t19766;
    (t19754, t19756, t19758, t19760, t19762, t19764, t19766, t19768)
}
