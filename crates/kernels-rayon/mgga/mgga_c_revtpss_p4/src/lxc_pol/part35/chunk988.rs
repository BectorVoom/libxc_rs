//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 988/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk988(t24348: f64, t24361: f64, t1169: f64, t12472: f64, t24330: f64, t1756: f64, t6518: f64) -> (f64, f64, f64) {
    let t24362 = t24348 + t24361;
    let t24363 = t24362 * t1169;
    let t24366 = t24330 * t12472;
    let t24375 = t6518 * t1756;
    (t24363, t24366, t24375)
}
