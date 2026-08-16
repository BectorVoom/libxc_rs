//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1104/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1104(t161: f64, t166: f64, t2093: f64, t6904: f64, t1848: f64, t2625: f64, t6596: f64, t831: f64, t16687: f64, t16689: f64, t17719: f64, t1924: f64, t5068: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t20279 = t161 * t166 * t2093 * t6904 / 10.0_f64;
    let t20281 = t1848 * t2625 / 10.0_f64;
    let t20283 = t831 * t6596 / 10.0_f64;
    let t20284 = 2.0_f64 / 27.0_f64 * t16687;
    let t20285 = 16.0_f64 / 81.0_f64 * t16689;
    let t20288 = 2.0_f64 / 15.0_f64 * t5068 * t17719 * t1924;
    (t20279, t20281, t20283, t20284, t20285, t20288)
}
