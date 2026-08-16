//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1283/1303 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1283(t1458: f64, t576: f64, t106: f64, t9364: f64, t111: f64, t5363: f64, t6470: f64, t19449: f64, t112: f64, t20148: f64, t5449: f64, t671: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t33185 = t576 * t1458;
    let t45435 = 1.0_f64 / t9364 / t106;
    let t55353 = t5363 * t111;
    let t55388 = t6470 * t111;
    let t55943 = t19449 * t111;
    let t66958 = t20148 * t112;
    let t75560 = t5449 * t671;
    (t33185, t45435, t55353, t55388, t55943, t66958, t75560)
}
