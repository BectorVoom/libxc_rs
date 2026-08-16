//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 327/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk327(t169: f64, t411: f64, t203: f64, t590: f64, t121: f64, t528: f64) -> (f64, f64, f64, f64) {
    let t1410 = t411 * t169;
    let t1411 = t1410 * t203;
    let t1412 = t1411 * t590;
    let t1415 = t528 * t121;
    (t1410, t1411, t1412, t1415)
}
