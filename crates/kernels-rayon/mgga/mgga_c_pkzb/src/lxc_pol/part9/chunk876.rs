//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 876/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk876(t179: f64, t6405: f64, t6406: f64, t2370: f64, t824: f64, t2434: f64, t2381: f64, t2029: f64, t919: f64, t2387: f64, t406: f64, t931: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6408 = t179 * t6405 * t6406;
    let t6411 = t2370 * t824;
    let t6412 = t2434 * t6411;
    let t6413 = t2381 * t6412;
    let t6416 = t919 * t2029;
    let t6417 = t2370 * t2387;
    let t6418 = t6416 * t6417;
    let t6419 = t406 * t6418;
    let t6422 = t931 * t824;
    (t6408, t6412, t6413, t6416, t6417, t6418, t6419, t6422)
}
