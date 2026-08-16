//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 736/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk736(t2093: f64, t2182: f64, t116: f64, t627: f64, t6560: f64, t673: f64, t686: f64, t6904: f64, t695: f64, t7034: f64, t7038: f64, t7040: f64, t7043: f64, t7046: f64, t7048: f64, t705: f64, t7051: f64, t7055: f64, t7062: f64) -> (f64, f64) {
    let t7064 = t2182 * t2093;
    let t7067 = t627 * t116 * t6560;
    let t7070 = 0.60852130428521304981e0_f64 * t7034 - 0.12170426085704260996e1_f64 * t7038 + 0.45342634012527777558e0_f64 * t705 * t7040 + 0.15647690681619764138e1_f64 * t686 * t7043 + 0.2115989587251296286e0_f64 * t7046 - 0.15114211337509259186e-1_f64 * t695 * t7048 + 0.60852130428521304981e0_f64 * t7051 - 0.86931614897887578546e-1_f64 * t673 * t7055 - 0.30228422675018518372e-1_f64 * t705 * t6904 - 0.23981215322181357908e1_f64 * t7062 + 0.4231979174502592572e0_f64 * t7064 - 0.17386322979577515709e0_f64 * t686 * t7067;
    (t7067, t7070)
}
