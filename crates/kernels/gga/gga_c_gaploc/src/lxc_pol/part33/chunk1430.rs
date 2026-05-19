//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1430/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1430<F: Float>(t12237: F, t33455: F, t33458: F, t33460: F, t33462: F, t33465: F, t33469: F, t33474: F, t33476: F, t33478: F, t33480: F, t33483: F, t33486: F, t33493: F, t33495: F, t33497: F, t5676: F) -> F {
    let t39140 = t33455 - t33458 + t33460 - t33462 - t33465 + t33469 - t33474 + t33476 + t33478 - t33480 - t33483 - t33486 + t33493 - t33495 + F::cast_from(0.79445533226334281486e-1_f64) * t5676 * t12237 - t33497;
    t39140
}
