//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 632/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk632(t10140: f64, t1457: f64, t1572: f64, t3395: f64, t6985: f64, t2487: f64, t10241: f64, t1339: f64, t590: f64, t1537: f64, t493: f64, t1441: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10463 = t1457 * t10140;
    let t10465 = 0.71500979903700853338e0_f64 * t1572 * t10463;
    let t10466 = t6985 * t3395;
    let t10467 = t2487 * t10466;
    let t10468 = 0.25561950635947166451e0_f64 * t10467;
    let t10469 = t1339 * t10241;
    let t10470 = t10469 * t590;
    let t10472 = 0.25561950635947166451e1_f64 * t1537 * t10470;
    let t10473 = t493 * t10241;
    let t10474 = t10473 * t590;
    let t10476 = 0.1022478025437886658e1_f64 * t1441 * t10474;
    (t10465, t10468, t10469, t10472, t10473, t10476)
}
