//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 80/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk80<F: Float>(t143: F, t147: F, t151: F, t154: F, t157: F, t160: F, t163: F, t166: F, t169: F, t172: F, t187: F) -> (F,) {
    let t144 = 0.135e1 <= t143;
    let t191 = piecewise3(t144, 1.0 / t147 / 36.0 - t151 / 960.0 + t154 / 26880.0 - t157 / 829440.0 + t160 / 28385280.0 - t163 / 0.107347968e10 + t166 / 0.445906944e11 - t169 / 0.20214448128e13, 1.0 - 8.0 / 3.0 * t172 * t187);
    (t191,)
}
