//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2854/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2854(t190: f64, t706: f64, t76397: f64, t40092: f64, t40094: f64, t14330: f64, t18305: f64, t4181: f64, t61201: f64, t157: f64, t23121: f64, t606: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t76986 = 4.0_f64 * t706 * t190 * t76397;
    let t76987 = 0.51947577317044391277e2_f64 * t40092;
    let t76988 = 0.35089341735807877242e1_f64 * t40094;
    let t76991 = 72.0_f64 * t14330 * t18305 * t4181;
    let t76992 = 24.0_f64 * t61201;
    let t76995 = 24.0_f64 * t606 * t157 * t23121;
    (t76986, t76987, t76988, t76991, t76992, t76995)
}
