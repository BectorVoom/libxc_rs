//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 971/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk971(t4608: f64, t582: f64, t4573: f64, t7737: f64, t581: f64, t3431: f64, t3446: f64, t2009: f64, t4579: f64, t13335: f64, t48: f64, t7750: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13365 = t582 * t4608;
    let t13370 = t7737 * t4573;
    let t13371 = t13370 * t581;
    let t13374 = t3446 * t3431;
    let t13379 = t2009 * t4579;
    let t13380 = t13379 * t581;
    let t13383 = t48 * t13335;
    let t13392 = t7750 * t4573;
    (t13365, t13371, t13374, t13380, t13383, t13392)
}
