//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1067/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1067(t1: f64, t2570: f64, t1830: f64, t453: f64, t350: f64, t7486: f64, t7494: f64, t1863: f64, t5961: f64, t36: f64, t2381: f64, t4667: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t19770 = t2570 * t1;
    let t19772 = t1830 * t453 * t19770;
    let t19774 = t350 * t7486;
    let t19776 = t350 * t7494;
    let t19778 = t1863 * t5961;
    let t19780 = t36 * t453 * t19778;
    let t19782 = t4667 * t2381;
    (t19770, t19772, t19774, t19776, t19778, t19780, t19782)
}
