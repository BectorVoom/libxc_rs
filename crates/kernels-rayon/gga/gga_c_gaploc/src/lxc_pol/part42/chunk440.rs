//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 440/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk440(t2610: f64, t486: f64, t1423: f64, t723: f64, t835: f64, t2102: f64, t773: f64, t2086: f64, t805: f64, t119: f64, t3831: f64, t481: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6118 = t486 * t2610;
    let t6119 = t1423 * t6118;
    let t6125 = t835 * t723;
    let t6141 = t773 * t2102;
    let t6148 = t805 * t2086;
    let t6305 = t481 * t3831 * t119;
    (t6118, t6119, t6125, t6141, t6148, t6305)
}
