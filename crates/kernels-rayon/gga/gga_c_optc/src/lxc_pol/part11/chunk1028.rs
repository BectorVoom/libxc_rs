//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1028/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1028(t2224: f64, t10195: f64, t10345: f64, t179: f64, t182: f64, t183: f64, t2211: f64, t2213: f64, t2217: f64, t2218: f64, t2219: f64, t23315: f64, t6576: f64, t6578: f64, t6581: f64, t6586: f64, t6587: f64, t6588: f64, t6589: f64, t6592: f64, t6597: f64, t720: f64, t723: f64, t724: f64, t727: f64) -> f64 {
    let t23321 = t2224 * t2224;
    let t23331 = (0.20106419753086419753e2_f64 * t10195 + 0.20068888888888888889e-1_f64 * t10345) * t183 - 4.0_f64 * t6576 * t723 * t727 + 12.0_f64 * t2211 * t2217 * t2219 - 6.0_f64 * t6578 * t2224 - 24.0_f64 * t720 * t6587 * t6589 + 24.0_f64 * t6581 * t6592 - 4.0_f64 * t2213 * t6597 + 24.0_f64 * t179 / t6586 / t182 * t23315 - 36.0_f64 * t6588 * t2219 * t2224 + 6.0_f64 * t2218 * t23321 + 8.0_f64 * t2218 * t727 * t6597 - t724 * (0.75383950617283950617e4_f64 * t10195 + 0.12819753086419753086e4_f64 * t10345);
    t23331
}
