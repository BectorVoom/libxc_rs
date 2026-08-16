//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 732/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk732(t36: f64, t6802: f64, t1476: f64, t6503: f64, t6512: f64, t1830: f64, t2546: f64, t350: f64, t506: f64, t6402: f64, t6406: f64, t2550: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6803 = t36 * t6802;
    let t6805 = t1476 * t6503;
    let t6806 = t36 * t6805;
    let t6808 = t1476 * t6512;
    let t6809 = t1830 * t6808;
    let t6811 = t350 * t2546;
    let t6813 = t506 * t6402;
    let t6814 = t36 * t6813;
    let t6816 = t506 * t6406;
    let t6817 = t1830 * t6816;
    let t6819 = t350 * t2550;
    (t6803, t6805, t6806, t6808, t6809, t6811, t6813, t6814, t6816, t6817, t6819)
}
