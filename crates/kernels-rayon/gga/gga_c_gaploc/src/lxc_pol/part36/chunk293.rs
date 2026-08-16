//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 293/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk293(t2366: f64, t475: f64, t2365: f64, t1429: f64, t1: f64, t2299: f64, t544: f64, t2339: f64, t549: f64, t2334: f64, t1445: f64, t2345: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2367 = t2366 * t475;
    let t2368 = t2365 * t2367;
    let t2369 = t1429 * t2368;
    let t2371 = t2299 * t1;
    let t2372 = t544 * t2371;
    let t2375 = t549 * t2339;
    let t2378 = t2334 * t475;
    let t2379 = t1445 * t2378;
    let t2382 = t1445 * t2345;
    (t2369, t2371, t2372, t2375, t2379, t2382)
}
