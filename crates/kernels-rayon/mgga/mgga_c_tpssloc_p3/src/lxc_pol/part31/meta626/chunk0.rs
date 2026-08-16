//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1883/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1883(t19894: f64, t22833: f64, t19886: f64, t5293: f64, t91100: f64, t19991: f64, t19882: f64, t16311: f64, t3788: f64, t5286: f64, t6936: f64, t28101: f64, t80958: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t97223 = t22833 * t19894;
    let t97225 = t22833 * t19886;
    let t97227 = t91100 * t5293;
    let t97229 = t22833 * t19991;
    let t97231 = t22833 * t19882;
    let t97236 = t6936 * t3788 * t16311 * t5286;
    let t97238 = t80958 * t28101;
    (t97223, t97225, t97227, t97229, t97231, t97236, t97238)
}
