//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2123/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2123(t18495: f64, t7045: f64, t18500: f64, t18618: f64, t7038: f64, t18466: f64, t25270: f64, t103302: f64, t103305: f64, t92996: f64, t92998: f64, t93000: f64, t93001: f64, t93008: f64, t93013: f64, t93016: f64) -> f64 {
    let t106068 = t7045 * t18495;
    let t106070 = t7045 * t18500;
    let t106072 = t7038 * t18618;
    let t106074 = t25270 * t18466;
    let t106078 = -0.51448821741683684367e-1_f64 * t106068 + 0.17149607247227894789e-1_f64 * t106070 - 0.42874018118069736972e-3_f64 * t106072 + t103302 - t92996 - t103305 - 0.42874018118069736972e-3_f64 * t106074 - t92998 + t93000 - 0.60976381323476959248e-3_f64 * t93001 + t93008 - t93013 - 0.90357964994909313586e-5_f64 * t93016;
    t106078
}
