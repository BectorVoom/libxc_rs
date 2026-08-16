//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1174/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1174(t31610: f64, t31612: f64, t31614: f64, t31615: f64, t31617: f64, t31618: f64, t31619: f64, t31620: f64, t31621: f64, t31622: f64) -> f64 {
    let t31623 = t31610 - t31612 + t31614 + t31615 / 2.0_f64 + t31617 - t31618 + t31619 - t31620 + t31621 - t31622;
    t31623
}
