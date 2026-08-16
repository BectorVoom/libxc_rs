//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 481/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk481(t153: f64, t2517: f64, t145: f64, t2447: f64, t185: f64, t193: f64, t2373: f64, t2377: f64, t2378: f64, t2379: f64, t2408: f64, t2417: f64, t2423: f64, t2426: f64, t2429: f64, t2432: f64, t2450: f64) -> (f64, f64, f64, f64) {
    let t2518 = t153 * t2517;
    let t2519 = t145 * t2447;
    let t2520 = t2519 * t185;
    let t2521 = 6.0_f64 * t193 * t2378 * t2379 + t2373 + t2377 + t2408 + t2417 - t2423 - t2426 + t2429 + t2432 + t2450 + t2518 + t2520;
    (t2518, t2519, t2520, t2521)
}
