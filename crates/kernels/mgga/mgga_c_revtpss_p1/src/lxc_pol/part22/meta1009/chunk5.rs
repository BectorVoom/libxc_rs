//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3457/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3457<F: Float>(t41361: F, t41363: F, t51973: F, t51978: F, t63325: F, t63328: F, t63336: F, t63338: F, t63340: F, t63342: F, t63346: F, t63351: F, t63355: F) -> F {
    let t65026 = -F::cast_from(0.14814814814814814815e-1_f64) * t51973 + F::cast_from(0.17283950617283950617e-1_f64) * t51978 + F::cast_from(0.17283950617283950617e-1_f64) * t41361 + F::cast_from(0.74074074074074074074e-2_f64) * t41363 - F::cast_from(0.37037037037037037036e-1_f64) * t63325 + F::cast_from(0.13333333333333333333e0_f64) * t63328 + F::cast_from(0.2e0_f64) * t63336 - F::cast_from(0.22222222222222222222e-1_f64) * t63338 + F::cast_from(0.74074074074074074073e-2_f64) * t63340 + F::cast_from(0.61728395061728395061e-2_f64) * t63342 - F::cast_from(0.92592592592592592592e-2_f64) * t63346 - F::cast_from(0.24691358024691358025e-1_f64) * t63351 + F::cast_from(0.33333333333333333333e-1_f64) * t63355;
    t65026
}
