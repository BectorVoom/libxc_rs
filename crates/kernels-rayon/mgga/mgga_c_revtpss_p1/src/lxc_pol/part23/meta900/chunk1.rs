//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2862/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2862(t50901: f64, t40076: f64, t40079: f64, t40194: f64, t40198: f64, t77036: f64, t77038: f64, t77039: f64, t77040: f64, t77041: f64, t77045: f64, t77048: f64, t77051: f64, t77053: f64, t77056: f64, t77058: f64, t77059: f64) -> (f64, f64) {
    let t77060 = 0.97592231702715658578e-1_f64 * t50901;
    let t77061 = t77036 + t77038 + t77039 + t77040 + t77041 + t77045 - t77048 + t77051 + t77053 + t77056 + t77058 + t40076 - t40079 + t40194 + t40198 - t77059 - t77060;
    (t77060, t77061)
}
