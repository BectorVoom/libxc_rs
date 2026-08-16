//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 828/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk828(t109: f64, t370: f64, t2247: f64, t2249: f64, t3643: f64, t5749: f64, t5752: f64, t5755: f64, t5762: f64, t5785: f64, t5789: f64, t5801: f64, t5804: f64, t5810: f64, t5852: f64, t5855: f64, t69: f64) -> (f64, f64) {
    let t5858 = t109 * t370;
    let t5860 = t2247 * t5858 * t2249;
    let t5862 = t5749 + t5752 - t5755 - t3643 - 0.7663355555555555_f64 * t5852 + t5855 - 1.724255_f64 * t69 * t5810 - t5762 - t5785 - 3.44851_f64 * t5860 - t5789 + t5801 + t5804;
    (t5858, t5862)
}
