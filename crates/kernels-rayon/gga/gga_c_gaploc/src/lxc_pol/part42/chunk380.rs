//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 380/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk380(t2859: f64, t3377: f64, t2366: f64, t986: f64, t2365: f64, t1429: f64, t1457: f64, t3354: f64, t1572: f64, t2778: f64, t874: f64, t1445: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3379 = 0.10725146985555128001e1_f64 * t2859 * t3377;
    let t3380 = t2366 * t986;
    let t3381 = t2365 * t3380;
    let t3382 = t1429 * t3381;
    let t3383 = 0.14896037479937677779e-1_f64 * t3382;
    let t3384 = t1457 * t3354;
    let t3386 = 0.71500979903700853338e0_f64 * t1572 * t3384;
    let t3390 = t2778 * t874;
    let t3391 = t1445 * t3390;
    (t3379, t3380, t3381, t3382, t3383, t3384, t3386, t3390, t3391)
}
