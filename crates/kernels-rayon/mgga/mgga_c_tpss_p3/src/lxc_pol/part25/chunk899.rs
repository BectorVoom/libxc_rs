//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 899/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk899(t8660: f64, t2617: f64, t314: f64, t8664: f64, t2574: f64, t872: f64, t2573: f64, t301: f64, t296: f64, t875: f64, t2576: f64, t304: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8756 = 0.28842592592592592592e-1_f64 * t8660;
    let t8772 = 1.0_f64 / t2617 / t314;
    let t8796 = 0.93932222222222222223e0_f64 * t8660;
    let t8797 = 0.36793333333333333333e0_f64 * t8664;
    let t8842 = t872 * t2574;
    let t8846 = 1.0_f64 / t2573 / t301;
    let t8847 = t296 * t8846;
    let t8871 = 0.16068111111111111111e1_f64 * t8660;
    let t8872 = 0.46308888888888888888e0_f64 * t8664;
    let t8887 = 1.0_f64 / t2573 / t875;
    let t8888 = t296 * t8887;
    let t8890 = 1.0_f64 / t2576 / t304;
    (t8756, t8772, t8796, t8797, t8842, t8847, t8871, t8872, t8888, t8890)
}
