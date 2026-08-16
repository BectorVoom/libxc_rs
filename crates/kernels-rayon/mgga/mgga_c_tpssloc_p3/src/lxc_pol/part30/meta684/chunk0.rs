//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2153/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2153(t26308: f64, t5234: f64, t5252: f64, t6396: f64, t80820: f64, t19962: f64, t22833: f64, t19894: f64, t19886: f64, t5293: f64, t91100: f64, t19991: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t97217 = t5234 * t26308 * t5252;
    let t97219 = t80820 * t6396;
    let t97221 = t22833 * t19962;
    let t97223 = t22833 * t19894;
    let t97225 = t22833 * t19886;
    let t97227 = t91100 * t5293;
    let t97229 = t22833 * t19991;
    (t97217, t97219, t97221, t97223, t97225, t97227, t97229)
}
