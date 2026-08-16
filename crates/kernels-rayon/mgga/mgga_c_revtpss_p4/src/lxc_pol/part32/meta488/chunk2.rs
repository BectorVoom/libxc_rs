//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1739/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1739(t27246: f64, t27251: f64, t27254: f64, t27256: f64, t25224: f64, t25230: f64, t25236: f64, t25279: f64, t26457: f64, t26462: f64, t26468: f64, t26471: f64, t27244: f64, t27249: f64, t27262: f64) -> f64 {
    let t28333 = 7.0_f64 / 72.0_f64 * t27246;
    let t28335 = 0.2032800112371413129e-3_f64 * t27251;
    let t28336 = 0.28582678745379824648e-4_f64 * t27254;
    let t28337 = 0.16006300097412701803e-1_f64 * t27256;
    let t28339 = t25279 - t26471 - t27244 / 24.0_f64 + t28333 - t25236 + t26457 + t26468 - 0.34299214494455789578e-2_f64 * t27249 - t28335 + t28336 + t28337 + t26462 + t25224 + t25230 + 0.17149607247227894789e-2_f64 * t27262;
    t28339
}
