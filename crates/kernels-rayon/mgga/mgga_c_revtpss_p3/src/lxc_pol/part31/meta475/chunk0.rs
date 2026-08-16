//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1742/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1742(t1927: f64, t644: f64, t1926: f64, t531: f64, t7311: f64, t1962: f64, t198: f64, t206: f64) -> (f64, f64, f64, f64) {
    let t25163 = t1927 * t644;
    let t25164 = t1926 * t25163;
    let t25190 = t531 * t7311;
    let t25206 = t198 * t206 * t1962;
    (t25163, t25164, t25190, t25206)
}
