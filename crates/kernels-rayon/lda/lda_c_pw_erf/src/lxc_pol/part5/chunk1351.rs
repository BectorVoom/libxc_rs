//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1351/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1351(t1: f64, t3: f64, t604: f64, t7337: f64, t22843: f64, t22844: f64, t22847: f64, t22850: f64, t22853: f64, t22857: f64, t22859: f64, t22860: f64, t22861: f64, t22862: f64, t22863: f64, t22868: f64) -> f64 {
    let t23321 = t7337 * t1 * t3 * t604;
    let t23323 = t22843 - t22844 + t22847 - t22850 + t22853 + t22857 - t22859 + t22860 + t22861 - t22862 + t22863 + 0.10821041362364843_f64 * t23321 + t22868;
    t23323
}
