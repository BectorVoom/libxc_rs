//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3416/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3416<F: Float>(t41281: F, t41285: F, t41287: F, t41672: F, t51937: F, t51942: F, t63266: F, t63268: F, t63274: F, t63276: F, t63278: F, t63281: F, t63285: F, t63290: F, t63293: F) -> F {
    let t64212 = -F::cast_from(0.27785333333333333334e0_f64) * t51937 + F::cast_from(0.83356000000000000002e0_f64) * t51942 + t41672 + F::cast_from(0.10589175e2_f64) * t63266 - F::cast_from(0.6311625e0_f64) * t63268 + F::cast_from(0.23154444444444444444e0_f64) * t41281 - F::cast_from(0.11577222222222222222e0_f64) * t41285 - F::cast_from(0.3859074074074074074e-1_f64) * t41287 + F::cast_from(0.20659e1_f64) * t63274 - F::cast_from(0.68863333333333333333e0_f64) * t63276 + F::cast_from(0.22954444444444444444e0_f64) * t63278 - F::cast_from(0.68863333333333333334e0_f64) * t63281 - F::cast_from(0.34431666666666666667e0_f64) * t63285 - F::cast_from(0.57386111111111111112e0_f64) * t63290 + F::cast_from(0.20659e1_f64) * t63293;
    t64212
}
