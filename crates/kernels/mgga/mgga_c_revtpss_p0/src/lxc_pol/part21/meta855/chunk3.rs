//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3238/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3238<F: Float>(t1298: F, t3794: F, t18134: F, t5023: F, t57822: F, t57825: F, t57827: F, t57829: F, t57831: F, t57833: F, t57835: F, t57837: F, t57840: F, t57842: F) -> F {
    let t60126 = t3794 * t1298;
    let t60130 = F::new(6.0) * t18134 * t5023 * t60126 - t57822 - t57825 + t57827 - t57829 - t57831 - t57833 + t57835 + t57837 - t57840 + t57842;
    t60130
}
