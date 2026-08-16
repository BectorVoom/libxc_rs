//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 983/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk983(t74948: f64, t14710: f64, t2868: f64, t739: f64, t7799: f64, t9530: f64, t74953: f64, t74957: f64, t3351: f64, t498: f64, t515: f64, t7248: f64, t9523: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t77281 = 0.2553875993597870364e-4_f64 * t74948;
    let t77282 = t2868 * t14710;
    let t77283 = 0.2993560425465952141e-1_f64 * t77282;
    let t77286 = 0.11974241701863808564e0_f64 * t739 * t9530 * t7799;
    let t77287 = 0.2553875993597870364e-4_f64 * t74953;
    let t77288 = 0.7661627980793611092e-4_f64 * t74957;
    let t77292 = t3351 * t7248 * t515 * t9523 * t498;
    (t77281, t77283, t77286, t77287, t77288, t77292)
}
