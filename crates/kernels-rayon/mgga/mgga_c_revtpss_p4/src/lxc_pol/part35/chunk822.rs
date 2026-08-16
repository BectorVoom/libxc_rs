//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 822/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk822(t221: f64, t2485: f64, t5978: f64, t2484: f64, t5819: f64, t750: f64, t2611: f64, t5825: f64, t706: f64, t4305: f64, t4311: f64, t5941: f64, t72: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18531 = t2485 * t221 * t5978;
    let t18532 = t2484 * t18531;
    let t18539 = t750 * t5819;
    let t18540 = t2611 * t18539;
    let t18544 = t750 * t5825;
    let t18545 = t706 * t18544;
    let t18547 = t4311 * t4305;
    let t18555 = t5941 * t72;
    (t18531, t18532, t18540, t18545, t18547, t18555)
}
