//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 707/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk707<F: Float>(t675: F, t7054: F, t155: F, t158: F, t2078: F, t661: F, t2093: F, t2182: F, t116: F, t627: F, t6560: F, t673: F, t686: F, t6904: F, t695: F, t7034: F, t7038: F, t7040: F, t7043: F, t7046: F, t7048: F, t705: F, t7051: F) -> (F, F, F, F) {
    let t7055 = t675 * t7054;
    let t7061 = t155 * t158 * t2078;
    let t7062 = t7061 * t661;
    let t7064 = t2182 * t2093;
    let t7067 = t627 * t116 * t6560;
    let t7070 = 0.60852130428521304981e0 * t7034 - 0.12170426085704260996e1 * t7038 + 0.45342634012527777558e0 * t705 * t7040 + 0.15647690681619764138e1 * t686 * t7043 + 0.2115989587251296286e0 * t7046 - 0.15114211337509259186e-1 * t695 * t7048 + 0.60852130428521304981e0 * t7051 - 0.86931614897887578546e-1 * t673 * t7055 - 0.30228422675018518372e-1 * t705 * t6904 - 0.23981215322181357908e1 * t7062 + 0.4231979174502592572e0 * t7064 - 0.17386322979577515709e0 * t686 * t7067;
    (t7055, t7061, t7067, t7070)
}
