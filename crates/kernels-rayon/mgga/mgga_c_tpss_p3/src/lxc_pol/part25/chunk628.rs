//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 628/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk628(t2457: f64, t3749: f64, t128: f64, t1289: f64, t2464: f64, t581: f64) -> (f64, f64, f64, f64) {
    let t3750 = t2457 * t3749;
    let t3751 = t128 * t3750;
    let t3753 = t2464 * t1289;
    let t3754 = t3753 * t581;
    (t3750, t3751, t3753, t3754)
}
