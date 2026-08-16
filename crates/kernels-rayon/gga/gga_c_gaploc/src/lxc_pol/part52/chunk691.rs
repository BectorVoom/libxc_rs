//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 691/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk691(t13397: f64, t2488: f64, t2487: f64, t123: f64, t3529: f64, t883: f64, t912: f64, t587: f64, t13261: f64, t1457: f64, t1572: f64, t11318: f64, t874: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13398 = t2488 * t13397;
    let t13399 = t2487 * t13398;
    let t13401 = t3529 * t123;
    let t13402 = t13401 * t883;
    let t13403 = t912 * t13402;
    let t13404 = t587 * t13403;
    let t13405 = 0.19171462976960374838e0_f64 * t13404;
    let t13415 = t1457 * t13261;
    let t13417 = 0.71500979903700853338e0_f64 * t1572 * t13415;
    let t13420 = t11318 * t874;
    (t13398, t13399, t13401, t13402, t13403, t13405, t13415, t13417, t13420)
}
