//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1274/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1274(t1325: f64, t5237: f64, t7701: f64, t1997: f64, t6198: f64, t18438: f64, t18444: f64, t18446: f64, t18449: f64, t1318: f64, t4776: f64, t549: f64, t7408: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t22856 = t1325 * t5237 * t7701;
    let t22857 = 16.0_f64 / 27.0_f64 * t22856;
    let t22859 = 4.0_f64 / 15.0_f64 * t6198 * t1997;
    let t22860 = 64.0_f64 / 81.0_f64 * t18438;
    let t22861 = 32.0_f64 / 27.0_f64 * t18444;
    let t22862 = 32.0_f64 / 45.0_f64 * t18446;
    let t22863 = 32.0_f64 / 15.0_f64 * t18449;
    let t22868 = 64.0_f64 / 81.0_f64 * t1318 * t4776 * t7408 * t549;
    (t22857, t22859, t22860, t22861, t22862, t22863, t22868)
}
