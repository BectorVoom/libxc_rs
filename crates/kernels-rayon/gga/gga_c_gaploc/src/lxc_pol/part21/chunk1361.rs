//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1361/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1361(t11986: f64, t1323: f64, t29923: f64, t31610: f64, t31612: f64, t31614: f64, t31617: f64, t31618: f64, t31619: f64, t31620: f64, t31621: f64, t31622: f64) -> (f64, f64) {
    let t38299 = t11986 * t1323;
    let t38313 = t31610 - t31612 + t31614 + t29923 + t31617 - t31618 + t31619 - t31620 + t31621 - t31622;
    (t38299, t38313)
}
