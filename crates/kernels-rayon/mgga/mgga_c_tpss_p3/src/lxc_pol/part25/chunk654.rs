//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 654/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk654(t2838: f64, t4047: f64, t128: f64, t1289: f64, t2845: f64, t581: f64) -> (f64, f64, f64, f64) {
    let t4048 = t2838 * t4047;
    let t4049 = t128 * t4048;
    let t4051 = t2845 * t1289;
    let t4052 = t4051 * t581;
    (t4048, t4049, t4051, t4052)
}
