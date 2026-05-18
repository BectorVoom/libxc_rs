//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 944/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk944<F: Float>(t1580: F, t184: F, t185: F, t21: F, t2236: F, t2240: F, t2301: F, t2306: F, t363: F, t37391: F, t39390: F, t39396: F, t39438: F, t39481: F, t39574: F, t39624: F, t5: F, t620: F, t623: F, t649: F, t650: F, t7745: F, t8614: F, t8723: F, t8731: F, t8732: F, t9471: F) -> F {
    let t39637 = F::new(3.0) / F::new(2.0) * t5 * t2236 * t1580 + t5 * t185 * t37391 / F::new(4.0) + t623 * t649 * t7745 + t5 * t620 * t7745 + F::new(3.0) / F::new(2.0) * t8614 * t2306 + t623 * t8731 * t363 + t623 * t39390 * t184 * t21 / F::new(4.0) + t2240 * t8732 + t39396 * t650 + t623 * (t39438 + t39481 + t39574 + t39624) * t184 * t21 / F::new(4.0) + t623 * t8723 * t363 + t5 * t9471 * t363 + F::new(3.0) / F::new(2.0) * t8614 * t2301;
    t39637
}
