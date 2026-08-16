//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1928/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1928(t25207: f64, t29598: f64, t1468: f64, t1544: f64, t30: f64, t5962: f64, t1579: f64, t7759: f64, t7071: f64, t25262: f64, t6024: f64, t25270: f64, t6037: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t29599 = t25207 * t29598;
    let t29602 = t1468 * t1544;
    let t29606 = t30 * t5962;
    let t29610 = t7759 * t1579;
    let t29611 = t7071 * t29610;
    let t29616 = t25262 * t6024;
    let t29618 = t25270 * t6037;
    (t29599, t29602, t29606, t29610, t29611, t29616, t29618)
}
