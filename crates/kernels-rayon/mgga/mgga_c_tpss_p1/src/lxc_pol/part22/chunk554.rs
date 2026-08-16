//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 554/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk554(t235: f64, t2376: f64, t238: f64, t242: f64, t232: f64, t339: f64, t789: f64, t795: f64) -> (f64, f64, f64, f64) {
    let t2377 = t2376 * t235;
    let t2379 = t2377 * t238 * t242;
    let t2381 = 119.0_f64 / 13824.0_f64 * t232 * t2379;
    let t2383 = t339 * t795 * t789;
    (t2377, t2379, t2381, t2383)
}
