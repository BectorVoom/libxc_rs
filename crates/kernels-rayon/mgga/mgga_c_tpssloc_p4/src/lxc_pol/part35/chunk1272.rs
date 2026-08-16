//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1272/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1272(t82218: f64, t10109: f64, t225: f64, t1914: f64, t40772: f64, t3034: f64, t336: f64, t221: f64, t697: f64, t1016: f64, t835: f64, t39063: f64, t7245: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t82219 = 0.27720185200590482541e0_f64 * t82218;
    let t82252 = t225 * t10109;
    let t82312 = t1914 * t40772;
    let t82510 = 1.0_f64 / t3034 / t336;
    let t82631 = t221 * t697;
    let t82985 = 1.0_f64 / t3034 / t1016;
    let t83803 = 1232.0_f64 / 27.0_f64 * t835;
    let t85501 = t39063 * t7245;
    (t82219, t82252, t82312, t82510, t82631, t82985, t83803, t85501)
}
