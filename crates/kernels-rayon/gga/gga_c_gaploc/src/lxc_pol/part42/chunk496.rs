//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 496/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk496(t2488: f64, t9278: f64, t2487: f64, t2344: f64, t2465: f64, t2464: f64, t1641: f64, t3193: f64, t2462: f64, t60: f64) -> (f64, f64, f64, f64) {
    let t9364 = t2488 * t9278;
    let t9365 = t2487 * t9364;
    let t9367 = t2465 * t2344;
    let t9368 = t2464 * t9367;
    let t9369 = t2487 * t9368;
    let t9371 = t1641 * t3193;
    let t9419 = t60 * t2462;
    (t9365, t9369, t9371, t9419)
}
