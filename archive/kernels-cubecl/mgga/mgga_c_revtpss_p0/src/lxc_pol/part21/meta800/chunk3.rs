//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2904/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2904<F: Float>(t51973: F, t41281: F, t41283: F, t41285: F, t41287: F, t41289: F, t41292: F, t41610: F, t51961: F, t51965: F, t51967: F, t51971: F) -> F {
    let t52701 = F::cast_from(0.39862222222222222223e0_f64) * t51973;
    let t52702 = F::cast_from(0.54771111111111111111e0_f64) * t41281 - F::cast_from(0.10954222222222222222e0_f64) * t41283 - F::cast_from(0.27385555555555555556e0_f64) * t41285 - F::cast_from(0.91285185185185185185e-1_f64) * t41287 + F::cast_from(0.54771111111111111111e-1_f64) * t41289 + F::cast_from(0.24342716049382716049e-1_f64) * t41292 + t41610 + F::cast_from(0.35876000000000000001e1_f64) * t51961 - F::cast_from(0.99655555555555555554e0_f64) * t51965 + F::cast_from(0.29896666666666666667e0_f64) * t51967 - F::cast_from(0.29896666666666666667e0_f64) * t51971 - t52701;
    t52702
}
