//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1226/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1226(t10143: f64, t25: f64, t2775: f64, t387: f64, t221: f64, t4509: f64, t1926: f64, t2770: f64, t3127: f64, t381: f64, t23602: f64, t1014: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t25373 = t10143 * t25;
    let t25423 = t387 * t2775;
    let t25428 = t221 * t4509;
    let t25429 = t1926 * t25428;
    let t25430 = t387 * t2770;
    let t25483 = t3127 * t381;
    let t25484 = t23602 * t25483;
    let t25490 = t1014 * t381;
    (t25373, t25423, t25429, t25430, t25484, t25490)
}
