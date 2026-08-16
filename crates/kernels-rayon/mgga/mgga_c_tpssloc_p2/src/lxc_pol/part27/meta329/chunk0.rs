//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1408/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1408(t3639: f64, t500: f64, t3696: f64, t588: f64, t592: f64, t1287: f64, t2223: f64, t1291: f64, t9874: f64, t25: f64, t514: f64, t28: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11947 = 1.0_f64 / t3639 / t500;
    let t11975 = t588 * t3696;
    let t11977 = t592 * t3696;
    let t11981 = t2223 * t1287;
    let t11984 = 0.56968947174242584612e-3_f64 * t1291 * t9874;
    let t11985 = t25 * t25;
    let t11987 = 1.0_f64 / t514 / t11985;
    let t11998 = t28 * t28;
    (t11947, t11975, t11977, t11981, t11984, t11987, t11998)
}
