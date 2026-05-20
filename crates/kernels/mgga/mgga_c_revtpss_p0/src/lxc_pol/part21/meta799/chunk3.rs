//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2896/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2896<F: Float>(t51973: F, t41281: F, t41283: F, t41285: F, t41287: F, t41289: F, t41292: F, t41690: F, t51961: F, t51965: F, t51967: F, t51971: F) -> F {
    let t52573 = F::cast_from(0.68863333333333333332e0_f64) * t51973;
    let t52574 = F::cast_from(0.69463333333333333333e0_f64) * t41281 - F::cast_from(0.13892666666666666667e0_f64) * t41283 - F::cast_from(0.34731666666666666666e0_f64) * t41285 - F::cast_from(0.11577222222222222222e0_f64) * t41287 + F::cast_from(0.69463333333333333333e-1_f64) * t41289 + F::cast_from(0.30872592592592592592e-1_f64) * t41292 + t41690 + F::new(0.61977e1) * t51961 - F::cast_from(0.17215833333333333333e1_f64) * t51965 + F::cast_from(0.51647499999999999999e0_f64) * t51967 - F::new(0.516475e0) * t51971 - t52573;
    t52574
}
