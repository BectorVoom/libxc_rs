//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 331/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk331(t2617: f64, t969: f64, t2615: f64, t590: f64, t948: f64, t1890: f64, t935: f64) -> (f64, f64, f64) {
    let t2618 = t969 * t2617;
    let t2619 = t2615 * t2618;
    let t2621 = t948 * t590;
    let t2624 = t1890 * t935;
    (t2619, t2621, t2624)
}
