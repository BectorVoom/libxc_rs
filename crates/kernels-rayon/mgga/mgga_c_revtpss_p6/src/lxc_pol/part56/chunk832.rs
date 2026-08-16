//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 832/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk832(t7769: f64, t886: f64, t25317: f64, t225: f64, t27265: f64, t1579: f64, t231: f64, t836: f64, t25392: f64, t7048: f64, t7071: f64, t7759: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t27299 = t7769 * t886;
    let t27300 = t25317 * t27299;
    let t27303 = t27265 * t225;
    let t27312 = t1579 * t836 * t231;
    let t27313 = t25392 * t27312;
    let t27316 = t7048 * t1579;
    let t27317 = t7071 * t27316;
    let t27322 = t7071 * t7759 * t886;
    (t27300, t27303, t27312, t27313, t27317, t27322)
}
