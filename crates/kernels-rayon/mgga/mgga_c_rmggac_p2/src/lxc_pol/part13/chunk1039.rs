//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1039/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1039(t38622: f64, t38639: f64, t38594: f64, t38599: f64, t38604: f64, t38606: f64, t38608: f64, t38610: f64, t38615: f64, t38617: f64, t38619: f64, t38624: f64, t38626: f64, t38628: f64, t38630: f64, t38632: f64, t38634: f64, t38636: f64) -> f64 {
    let t42685 = 0.49658699875514145965e-4_f64 * t38622;
    let t42693 = 0.39726959900411316772e-4_f64 * t38639;
    let t42694 = -0.5107751987195740728e-4_f64 * t38594 + 0.5107751987195740728e-4_f64 * t38599 + 0.1702583995731913576e-4_f64 * t38604 - 0.1702583995731913576e-4_f64 * t38606 - 0.30487649791575028312e-3_f64 * t38608 + 0.30487649791575028312e-3_f64 * t38610 - 0.1702583995731913576e-4_f64 * t38615 - 0.85129199786595678799e-5_f64 * t38617 + 0.1702583995731913576e-4_f64 * t38619 + t42685 + 0.2553875993597870364e-4_f64 * t38624 - 0.5107751987195740728e-4_f64 * t38626 - 0.5107751987195740728e-4_f64 * t38628 - 0.2553875993597870364e-4_f64 * t38630 - 0.1702583995731913576e-4_f64 * t38632 - 0.1702583995731913576e-4_f64 * t38634 - 0.85129199786595678799e-5_f64 * t38636 + t42693;
    t42694
}
