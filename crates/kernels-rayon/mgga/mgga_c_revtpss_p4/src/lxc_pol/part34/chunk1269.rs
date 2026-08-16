//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1269/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1269(t29508: f64, t7742: f64, t29502: f64, t7732: f64, t30123: f64, t98450: f64, t2014: f64, t22475: f64, t7934: f64, t29996: f64, t7898: f64, t30005: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t113063 = 6.0_f64 * t29508 * t7742;
    let t113065 = 12.0_f64 * t7732 * t29502;
    let t113067 = 18.0_f64 * t98450 * t30123;
    let t113076 = 6.0_f64 * t2014 * t7934 * t22475;
    let t113078 = 6.0_f64 * t7898 * t29996;
    let t113084 = 6.0_f64 * t7732 * t30005;
    (t113063, t113065, t113067, t113076, t113078, t113084)
}
