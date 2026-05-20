//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2740/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2740<F: Float>(t128: F, t121: F, t268: F, t8779: F, t588: F, t9295: F, t2508: F, t39494: F, t39497: F, t692: F, t124: F, t138: F, t239: F) -> (F, F, F, F, F) {
    let t39503 = F::powf(t128, -F::new(0.25e1));
    let t39506 = t39503 * t121 * t8779 * t268;
    let t39508 = t9295 * t588;
    let t39510 = t2508 * t39494;
    let t39512 = t692 * t39497;
    let t39515 = t138 * t124 * t239;
    (t39506, t39508, t39510, t39512, t39515)
}
