//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2471/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2471(t2857: f64, t5825: f64, t606: f64, t904: f64, t128: f64) -> (f64, f64, f64, f64) {
    let t18941 = t2857 * t5825;
    let t18942 = t18941 * t606;
    let t18943 = t904 * t18942;
    let t18944 = t128 * t18943;
    (t18941, t18942, t18943, t18944)
}
