//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1315/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1315(t110143: f64, t8185: f64, t29895: f64, t30060: f64, t29900: f64, t30067: f64, t111: f64, t8199: f64, t112: f64, t30094: f64, t1404: f64, t656: f64, t9576: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t110336 = t110143 * t8185;
    let t110338 = t29895 * t30060;
    let t110340 = t29900 * t30067;
    let t110363 = t8199 * t111;
    let t110376 = t30094 * t112;
    let t110484 = t8199 * t1404;
    let t110532 = t9576 * t656;
    (t110336, t110338, t110340, t110363, t110376, t110484, t110532)
}
