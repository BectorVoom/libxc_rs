//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 455/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk455(t2304: f64, t494: f64, t484: f64, t885: f64, t417: f64, t78: f64, t119: f64, t481: f64) -> (f64, f64, f64, f64) {
    let t2305 = t2304 * t494;
    let t2308 = t484 * t885;
    let t2310 = t78 * t417;
    let t2312 = t481 * t2310 * t119;
    (t2305, t2308, t2310, t2312)
}
