//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 905/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk905<F: Float>(t38622: F, t38639: F, t38594: F, t38599: F, t38604: F, t38606: F, t38608: F, t38610: F, t38615: F, t38617: F, t38619: F, t38624: F, t38626: F, t38628: F, t38630: F, t38632: F, t38634: F, t38636: F) -> (F,) {
    let t42685 = 0.49658699875514145965e-4 * t38622;
    let t42693 = 0.39726959900411316772e-4 * t38639;
    let t42694 = -0.5107751987195740728e-4 * t38594 + 0.5107751987195740728e-4 * t38599 + 0.1702583995731913576e-4 * t38604 - 0.1702583995731913576e-4 * t38606 - 0.30487649791575028312e-3 * t38608 + 0.30487649791575028312e-3 * t38610 - 0.1702583995731913576e-4 * t38615 - 0.85129199786595678799e-5 * t38617 + 0.1702583995731913576e-4 * t38619 + t42685 + 0.2553875993597870364e-4 * t38624 - 0.5107751987195740728e-4 * t38626 - 0.5107751987195740728e-4 * t38628 - 0.2553875993597870364e-4 * t38630 - 0.1702583995731913576e-4 * t38632 - 0.1702583995731913576e-4 * t38634 - 0.85129199786595678799e-5 * t38636 + t42693;
    (t42694,)
}
