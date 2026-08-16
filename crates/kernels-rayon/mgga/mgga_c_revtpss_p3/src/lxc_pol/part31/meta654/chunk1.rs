//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2187/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2187(t108076: f64, t108078: f64, t108080: f64, t108083: f64, t108085: f64, t108087: f64, t108089: f64, t108099: f64, t108103: f64, t108105: f64, t108107: f64, t108109: f64, t108111: f64, t18235: f64, t18242: f64, t25805: f64, t27145: f64, t28025: f64, t28053: f64, t4248: f64, t5921: f64, t6985: f64) -> f64 {
    let t108114 = -4.0_f64 * t18235 * t6985 - 2.0_f64 * t18242 * t6985 - 2.0_f64 * t25805 * t5921 - 4.0_f64 * t27145 * t4248 - 2.0_f64 * t28025 * t5921 - 4.0_f64 * t28053 * t4248 - t108076 - t108078 - t108080 - t108083 - t108085 - t108087 - t108089 - t108099 + t108103 - t108105 - t108107 - t108109 - t108111;
    t108114
}
