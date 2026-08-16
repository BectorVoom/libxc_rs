//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2232/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2232(t1510: f64, t22986: f64, t6646: f64, t87111: f64, t16820: f64, t1888: f64, t22996: f64, t17031: f64, t829: f64, t98389: f64, t16815: f64, t9627: f64) -> (f64, f64, f64, f64, f64) {
    let t98461 = t22986 * t6646 * t87111 * t1510;
    let t98464 = t1888 * t22996 * t16820;
    let t98467 = t1888 * t22996 * t17031;
    let t98471 = t22986 * t6646 * t98389 * t829;
    let t98475 = t22986 * t22996 * t16815 * t9627;
    (t98461, t98464, t98467, t98471, t98475)
}
