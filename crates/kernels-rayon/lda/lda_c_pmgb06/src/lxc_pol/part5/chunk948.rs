//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 948/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk948(t14297: f64, t1186: f64, t421: f64, t5613: f64, t1354: f64, t2841: f64, t4429: f64, t118: f64, t5567: f64, t11676: f64, t1366: f64, t5652: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14298 = 0.01975389032890948_f64 * t14297;
    let t14300 = t5613 * t1186 * t421;
    let t14303 = t4429 * t2841 * t1354;
    let t14306 = t5567 * t118;
    let t14308 = t11676 * t118;
    let t14310 = t5652 * t1366;
    (t14298, t14300, t14303, t14306, t14308, t14310)
}
