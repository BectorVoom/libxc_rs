//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 872/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk872(t652: f64, t8533: f64, t2047: f64, t225: f64, t258: f64, t214: f64, t1880: f64, t8340: f64, t8345: f64) -> (f64, f64, f64, f64, f64) {
    let t8535 = 2.0_f64 * t652 * t8533;
    let t8537 = t2047 * t225 * t258;
    let t8538 = t214 * t8537;
    let t8539 = t1880 * t8538;
    let t8543 = 0.16149102437656156341e-2_f64 * t8340 + t8345 / 768.0_f64;
    (t8535, t8537, t8538, t8539, t8543)
}
