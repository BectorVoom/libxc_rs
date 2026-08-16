//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2549/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2549(t63332: f64, t63334: f64, t63336: f64, t63886: f64, t63888: f64, t63893: f64, t71124: f64, t71130: f64, t71135: f64, t71140: f64, t71142: f64, t71391: f64) -> f64 {
    let t71571 = 0.99655555555555555555e0_f64 * t71124 - 0.26574814814814814815e0_f64 * t63332 + 0.39862222222222222223e0_f64 * t63334 - 0.29896666666666666667e0_f64 * t63336 - 0.35876e1_f64 * t71130 - 0.16431333333333333333e0_f64 * t63886 - 0.91285185185185185184e-1_f64 * t63888 + 0.5477111111111111111e0_f64 * t63893 + 0.3071625e0_f64 * t71391 + 0.39862222222222222223e1_f64 * t71135 - 0.19931111111111111111e0_f64 * t71140 + 0.19931111111111111111e0_f64 * t71142;
    t71571
}
