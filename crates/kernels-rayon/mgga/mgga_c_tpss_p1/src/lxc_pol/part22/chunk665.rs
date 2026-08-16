//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 665/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk665(t3110: f64, t1134: f64, t219: f64, t1137: f64, t471: f64, t73: f64, t1148: f64, t2711: f64, t2712: f64, t3048: f64, t3053: f64, t466: f64, param_beta: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3111 = param_beta * t3110;
    let t3113 = t1134 * t219;
    let t3117 = 1.0_f64 / t1137 / t471;
    let t3118 = t73 * t3117;
    let t3119 = t1148 * t1148;
    let t3120 = t3118 * t3119;
    let t3124 = t2711 * t2712 * t3048;
    let t3125 = t466 * t3053;
    (t3111, t3113, t3118, t3119, t3120, t3124, t3125)
}
