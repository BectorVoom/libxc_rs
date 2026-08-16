//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1167/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1167(t2014: f64, t29583: f64, t2034: f64, t22483: f64, t30: f64, t5966: f64, t1963: f64, t1544: f64, t1583: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t29585 = 6.0_f64 * t2014 * t29583;
    let t29589 = t2034 * t22483;
    let t29590 = t2014 * t29589;
    let t29591 = t30 * t5966;
    let t29592 = t1963 * t29591;
    let t29598 = t1544 * t1583;
    (t29585, t29589, t29590, t29591, t29592, t29598)
}
