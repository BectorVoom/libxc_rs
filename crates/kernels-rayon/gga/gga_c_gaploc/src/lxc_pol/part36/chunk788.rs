//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 788/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk788(t12506: f64, t1429: f64, t549: f64, t2492: f64, t9267: f64, t9278: f64, t2482: f64, t3133: f64, t3125: f64, t9263: f64, t1538: f64, t30208: f64, t6583: f64, t883: f64) -> (f64, f64, f64, f64, f64) {
    let t40283 = t1429 * t549 * t12506;
    let t40301 = t9267 * t2492 * t9278;
    let t40320 = t9267 * t3133 * t2482;
    let t40332 = t9263 * t3125 * t2482;
    let t40336 = t6583 * t1538 * t883 * t30208;
    (t40283, t40301, t40320, t40332, t40336)
}
