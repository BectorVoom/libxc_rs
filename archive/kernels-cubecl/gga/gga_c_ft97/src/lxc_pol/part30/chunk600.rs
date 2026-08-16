//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 600/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk600<F: Float>(t24438: F, t27757: F, t6118: F, t24455: F, t24470: F, t27466: F, t27471: F, t27473: F, t27477: F, t27481: F, t27485: F, t27745: F, t27751: F, t27755: F) -> (F, F) {
    let t27758 = t24438 * t27757;
    let t27759 = t6118 * t27758;
    let t27761 = t27466 / F::cast_from(6.0_f64) + t27471 / F::cast_from(3.0_f64) - t27473 / F::cast_from(9.0_f64) - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t27477 - F::cast_from(6.0_f64) * t27481 + t27485 / F::cast_from(3.0_f64) - t27745 / F::cast_from(2.0_f64) - t24455 / F::cast_from(12.0_f64) - t24470 / F::cast_from(3.0_f64) - F::cast_from(3.0_f64) * t27751 - t27755 / F::cast_from(3.0_f64) - t27759 / F::cast_from(3.0_f64);
    (t27759, t27761)
}
