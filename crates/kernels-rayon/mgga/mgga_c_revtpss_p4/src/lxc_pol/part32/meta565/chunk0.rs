//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1888/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1888(t2453: f64, t27212: f64, t1032: f64, t4469: f64, t867: f64, t786: f64, t1955: f64, t7063: f64, t1568: f64, t25410: f64, t25374: f64, t98848: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t99257 = t2453 * t27212;
    let t99270 = t4469 * t1032;
    let t99271 = t99270 * t867;
    let t99272 = t786 * t99271;
    let t99303 = t1955 * t99270;
    let t99373 = t7063 * t99271;
    let t99403 = t786 * t1568;
    let t99404 = t99403 * t25410;
    let t99463 = t98848 * t25374;
    (t99257, t99272, t99303, t99373, t99403, t99404, t99463)
}
