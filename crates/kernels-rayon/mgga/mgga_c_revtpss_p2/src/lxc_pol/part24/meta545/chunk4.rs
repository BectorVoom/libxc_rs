//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1616/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1616(t2723: f64, t87399: f64, t39419: f64, t39422: f64, t39429: f64, t39432: f64, t87262: f64, t87263: f64, t87265: f64, t87267: f64, t87268: f64, t87296: f64, t87298: f64) -> (f64, f64) {
    let t87629 = t87399 * t2723;
    let t87634 = t87262 + t87263 + t87265 - t39419 - t39422 + t87267 - t87268 + t87296 + t87298 - t39429 - t39432;
    (t87629, t87634)
}
