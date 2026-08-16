//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 736/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk736<F: Float>(t2093: F, t2182: F, t116: F, t627: F, t6560: F, t673: F, t686: F, t6904: F, t695: F, t7034: F, t7038: F, t7040: F, t7043: F, t7046: F, t7048: F, t705: F, t7051: F, t7055: F, t7062: F) -> (F, F) {
    let t7064 = t2182 * t2093;
    let t7067 = t627 * t116 * t6560;
    let t7070 = F::cast_from(0.60852130428521304981e0_f64) * t7034 - F::cast_from(0.12170426085704260996e1_f64) * t7038 + F::cast_from(0.45342634012527777558e0_f64) * t705 * t7040 + F::cast_from(0.15647690681619764138e1_f64) * t686 * t7043 + F::cast_from(0.2115989587251296286e0_f64) * t7046 - F::cast_from(0.15114211337509259186e-1_f64) * t695 * t7048 + F::cast_from(0.60852130428521304981e0_f64) * t7051 - F::cast_from(0.86931614897887578546e-1_f64) * t673 * t7055 - F::cast_from(0.30228422675018518372e-1_f64) * t705 * t6904 - F::cast_from(0.23981215322181357908e1_f64) * t7062 + F::cast_from(0.4231979174502592572e0_f64) * t7064 - F::cast_from(0.17386322979577515709e0_f64) * t686 * t7067;
    (t7067, t7070)
}
