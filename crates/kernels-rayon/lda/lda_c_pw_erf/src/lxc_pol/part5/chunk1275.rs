//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1275/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1275(t10379: f64, t571: f64, t593: f64, t7408: f64, t14257: f64, t21811: f64, t15123: f64, t15125: f64, t15138: f64, t15140: f64, t22857: f64, t22859: f64, t22860: f64, t22861: f64, t22862: f64, t22863: f64, t22868: f64) -> (f64, f64, f64) {
    let t22872 = 32.0_f64 / 81.0_f64 * t571 * t10379 * t7408 * t593;
    let t22875 = 352.0_f64 / 243.0_f64 * t571 * t14257 * t21811;
    let t22876 = t22857 - t22859 + t22860 + t22861 - t22862 + t22863 + t15123 + 0.547_f64 * t15125 + t15138 + t15140 + t22868 - t22872 + t22875;
    (t22872, t22875, t22876)
}
