//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 646/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk646(t3815: f64, t1788: f64, t588: f64, t592: f64, t3829: f64, t3833: f64, t2426: f64, t2486: f64, t3819: f64, t3821: f64, t3825: f64, t3827: f64, t3832: f64, t5169: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5263 = 0.18311447306006545054e-3_f64 * t3815;
    let t5264 = t588 * t1788;
    let t5265 = 4.0_f64 * t5264;
    let t5266 = t592 * t1788;
    let t5267 = 4.0_f64 * t5266;
    let t5268 = 4.0_f64 * t3829;
    let t5269 = 4.0_f64 * t3833;
    let t5270 = t5169 - t5263 - t2426 + t3819 - t3821 + t3825 + t5265 - t5267 + t3827 - t5268 - t2486 - t3832 - t5269;
    (t5263, t5265, t5267, t5268, t5269, t5270)
}
