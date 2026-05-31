//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 80/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk80<F: Float>(t143: F, t147: F, t151: F, t154: F, t157: F, t160: F, t163: F, t166: F, t169: F, t172: F, t187: F) -> F {
    let t144 = F::cast_from(0.135e1_f64) <= t143;
    let t191 = piecewise3::<F>(t144, F::cast_from(1.0_f64) / t147 / F::cast_from(36.0_f64) - t151 / F::cast_from(960.0_f64) + t154 / F::cast_from(26880.0_f64) - t157 / F::cast_from(829440.0_f64) + t160 / F::cast_from(28385280.0_f64) - t163 / F::cast_from(0.107347968e10_f64) + t166 / F::cast_from(0.445906944e11_f64) - t169 / F::cast_from(0.20214448128e13_f64), F::cast_from(1.0_f64) - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t172 * t187);
    t191
}
