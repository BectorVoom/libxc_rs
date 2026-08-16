//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 791/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk791(t3: f64, t5465: f64, t116: f64, t4637: f64, t117: f64, t4674: f64, t1668: f64, t1670: f64, t547: f64, t548: f64, t1976: f64, t38: f64, param_d: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5466 = t3 * t5465;
    let t5470 = param_d * t5465;
    let t5474 = t116 * t4637;
    let t5477 = t117 * t4674;
    let t5480 = 6.0_f64 * t1668 * t1670 + 6.0_f64 * t547 * t5474 + 3.0_f64 * t547 * t5477 + t5470 * t548;
    let t5483 = t1976 * t38;
    (t5466, t5470, t5474, t5477, t5480, t5483)
}
