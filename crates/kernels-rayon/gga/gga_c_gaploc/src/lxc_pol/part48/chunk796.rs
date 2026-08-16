//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 796/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk796(t12987: f64, t7014: f64, t2365: f64, t31558: f64, t7025: f64, t12943: f64, t4379: f64, t40452: f64, t10608: f64, t9272: f64, t9278: f64, t34600: f64, t544: f64, t9287: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t42256 = t7014 * t12987;
    let t42259 = t7025 * t2365 * t31558;
    let t42316 = t4379 * t12943;
    let t42341 = 0.31952438294933958063e0_f64 * t40452;
    let t42349 = t9272 * t10608 * t9278;
    let t42366 = t544 * t34600 * t9287;
    (t42256, t42259, t42316, t42341, t42349, t42366)
}
