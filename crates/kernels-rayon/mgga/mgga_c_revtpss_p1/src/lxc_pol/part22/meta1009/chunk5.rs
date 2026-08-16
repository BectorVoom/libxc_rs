//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3457/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3457(t41361: f64, t41363: f64, t51973: f64, t51978: f64, t63325: f64, t63328: f64, t63336: f64, t63338: f64, t63340: f64, t63342: f64, t63346: f64, t63351: f64, t63355: f64) -> f64 {
    let t65026 = -0.14814814814814814815e-1_f64 * t51973 + 0.17283950617283950617e-1_f64 * t51978 + 0.17283950617283950617e-1_f64 * t41361 + 0.74074074074074074074e-2_f64 * t41363 - 0.37037037037037037036e-1_f64 * t63325 + 0.13333333333333333333e0_f64 * t63328 + 0.2e0_f64 * t63336 - 0.22222222222222222222e-1_f64 * t63338 + 0.74074074074074074073e-2_f64 * t63340 + 0.61728395061728395061e-2_f64 * t63342 - 0.92592592592592592592e-2_f64 * t63346 - 0.24691358024691358025e-1_f64 * t63351 + 0.33333333333333333333e-1_f64 * t63355;
    t65026
}
