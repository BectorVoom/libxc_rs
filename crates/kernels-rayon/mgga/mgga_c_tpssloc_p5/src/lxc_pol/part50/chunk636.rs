//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 636/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk636(t25: f64, t184: f64, t5151: f64, t17: f64, t1787: f64, t750: f64, t1408: f64, t3704: f64, t1298: f64, t2: f64, t584: f64, t606: f64, t1649: f64, t3711: f64, zeta_threshold: f64) -> (f64, f64, f64, f64) {
    let t26 = t25 <= zeta_threshold;
    let t5166 = t5151 * t184;
    let t5167 = t17 * t5166;
    let t5168 = t1787 * t750;
    let t5169 = t17 * t5168;
    let t5170 = t3704 * t1408;
    let t5173 = t1298 * t2;
    let t5177 = piecewise3(t26, 0.0_f64, -2.0_f64 / 9.0_f64 * t5170 * t606 + 4.0_f64 / 3.0_f64 * t5173 * t584);
    let t5178 = t3711 * t1649;
    (t5167, t5169, t5177, t5178)
}
