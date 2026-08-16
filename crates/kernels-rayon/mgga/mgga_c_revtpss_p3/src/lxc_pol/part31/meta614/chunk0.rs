//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2057/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2057(t25411: f64, t98877: f64, t27349: f64, t689: f64, t92843: f64, t92838: f64, t27341: f64, t93342: f64, t93364: f64, t27194: f64, t887: f64, t1580: f64, t2439: f64, t25334: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t98881 = 0.25702851531048074406e-1_f64 * t25411 * t98877;
    let t98892 = t27349 * t689;
    let t98894 = 0.28912093960683998208e-1_f64 * t92843 * t98892;
    let t98897 = 0.51405703062096148812e-1_f64 * t92838 * t98892;
    let t98907 = 0.51405703062096148812e-1_f64 * t93342 * t27341;
    let t98911 = 0.28912093960683998208e-1_f64 * t93364 * t27341;
    let t98918 = 0.10975748638225852664e-1_f64 * t689 * t27194 * t887;
    let t98920 = t2439 * t25334 * t1580;
    (t98881, t98894, t98897, t98907, t98911, t98918, t98920)
}
