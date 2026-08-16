//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 576/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk576(t2381: f64, t2383: f64, t2191: f64, t2194: f64, t2201: f64, t2237: f64, t2245: f64, t2325: f64, t2327: f64, t2330: f64, t2334: f64, t2338: f64, t2342: f64) -> (f64, f64) {
    let t2384 = t2381 * t2383;
    let t2387 = -t2191 + t2194 - t2201 + t2237 + t2245 + t2325 + t2327 - t2330 + t2334 - t2338 - t2342;
    (t2384, t2387)
}
