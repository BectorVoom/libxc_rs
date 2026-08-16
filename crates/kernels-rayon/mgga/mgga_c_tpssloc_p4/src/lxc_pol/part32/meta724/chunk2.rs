//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2320/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2320(t19031: f64, t2139: f64, t471: f64, t24746: f64, t27607: f64, t8027: f64, t1409: f64, t1714: f64, t2132: f64, t52: f64, t6138: f64, t1222: f64, t29597: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t104107 = t471 * t2139 * t19031;
    let t104111 = t8027 * t27607 * t24746;
    let t104118 = t1409 * t1714;
    let t104120 = t2132 * t104118 * t24746;
    let t104122 = t52 * t6138;
    let t104124 = t2132 * t104122 * t24746;
    let t104126 = t29597 * t1222;
    (t104107, t104111, t104118, t104120, t104122, t104124, t104126)
}
