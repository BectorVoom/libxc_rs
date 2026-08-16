//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 739/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk739(t445: f64, t4779: f64, t439: f64, t1427: f64, t2002: f64, t1504: f64, t831: f64, t1848: f64, t490: f64, t4738: f64, t4739: f64, t4740: f64, t4756: f64, t4759: f64, t4764: f64, t4769: f64, t4771: f64, t4774: f64, t4776: f64, t4778: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4780 = t4779 * t445;
    let t4782 = 2.0_f64 / 45.0_f64 * t439 * t4780;
    let t4784 = 2.0_f64 / 45.0_f64 * t2002 * t1427;
    let t4786 = 2.0_f64 / 45.0_f64 * t831 * t1504;
    let t4788 = 2.0_f64 / 45.0_f64 * t1848 * t490;
    let t4789 = -t4738 - t4739 + 0.033245444444444446_f64 * t4740 + t4756 - t4759 - t4764 - t4769 - t4771 - t4774 - t4776 - t4778 + t4782 + t4784 + t4786 + t4788;
    (t4780, t4782, t4784, t4786, t4788, t4789)
}
