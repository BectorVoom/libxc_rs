//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1126/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1126(t27: f64, t82: f64, t2489: f64, t4805: f64, t16111: f64, t4795: f64, t973: f64, t1424: f64, t1429: f64, t440: f64, t2493: f64, t500: f64) -> (f64, f64, f64, f64, f64) {
    let t19418 = t27 * t82;
    let t19427 = t2489 * t4805;
    let t19435 = t16111 * t973 * t4795;
    let t19439 = t1424 * t1429 * t440;
    let t19442 = t2493 * t500;
    (t19418, t19427, t19435, t19439, t19442)
}
