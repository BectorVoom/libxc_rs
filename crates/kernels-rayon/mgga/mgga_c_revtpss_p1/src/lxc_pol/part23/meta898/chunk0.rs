//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2857/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2857(t61296: f64, t61305: f64, t39989: f64, t40150: f64, t50098: f64, t50866: f64, t77002: f64, t77003: f64, t77004: f64, t77005: f64, t77007: f64, t77008: f64, t77009: f64, t77010: f64, t77011: f64, t77012: f64, t77013: f64) -> (f64, f64, f64) {
    let t77014 = 0.51947577317044391276e2_f64 * t61296;
    let t77015 = 36.0_f64 * t61305;
    let t77016 = t77002 - t77003 + t77004 + t77005 + t77007 + t77008 + t50098 + t77009 - t39989 + t40150 + t77010 - t77011 - t77012 - t77013 - t77014 + t77015 + t50866;
    (t77014, t77015, t77016)
}
