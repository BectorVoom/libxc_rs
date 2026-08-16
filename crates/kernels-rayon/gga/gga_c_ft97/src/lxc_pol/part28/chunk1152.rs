//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1152/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1152(t3408: f64, t7339: f64, t1369: f64, t2112: f64, t28: f64, t34918: f64, t586: f64, t5890: f64, t590: f64, t446: f64, t5842: f64, t6630: f64, t9432: f64) -> (f64, f64, f64, f64) {
    let t148613 = t7339 * t3408;
    let t148616 = t1369 * t28 * t2112 * t148613;
    let t148621 = t5890 * t28 * t586 * t34918 * t590;
    let t148625 = t446 * t9432 * t6630 * t5842;
    (t148613, t148616, t148621, t148625)
}
