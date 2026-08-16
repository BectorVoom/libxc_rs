//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 510/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk510(t2663: f64, t756: f64, t2373: f64, t2377: f64, t2408: f64, t2417: f64, t2423: f64, t2426: f64, t2429: f64, t2432: f64, t2450: f64, t2486: f64, t2518: f64, t2520: f64, t2530: f64, t2533: f64, t2537: f64, t2539: f64, t2654: f64, t2657: f64, t2661: f64) -> (f64, f64) {
    let t2665 = 0.24415263074675393405e-3_f64 * t756 * t2663;
    let t2666 = -t2654 + t2373 + t2377 - t2486 + t2450 + t2518 + t2408 + t2417 + t2520 + t2539 - t2530 - t2533 - t2537 + t2657 + t2661 - t2426 + t2665 + t2429 + t2432 - t2423;
    (t2665, t2666)
}
