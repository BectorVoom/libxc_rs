//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 887/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk887(t235: f64, t8199: f64, t238: f64, t242: f64, t232: f64, t2215: f64, t2218: f64, t2345: f64, t2206: f64, t651: f64, t2348: f64, t123: f64, t727: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8200 = t8199 * t235;
    let t8202 = t8200 * t238 * t242;
    let t8204 = 595.0_f64 / 10368.0_f64 * t232 * t8202;
    let t8212 = t2218 * t2215;
    let t8218 = t2218 * t2345;
    let t8220 = t651 * t2206;
    let t8222 = 0.16265371950452609763e-1_f64 * t2348 * t8220;
    let t8223 = t651 * t2215;
    let t8225 = 0.48159733137676571078e0_f64 * t2348 * t8223;
    let t8226 = t727 * t123;
    (t8200, t8202, t8204, t8212, t8218, t8220, t8222, t8223, t8225, t8226)
}
