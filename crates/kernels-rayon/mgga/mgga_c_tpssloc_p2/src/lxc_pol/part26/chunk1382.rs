//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1382/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1382(t85442: f64, t85585: f64, t85595: f64, t86548: f64, t12529: f64, t12532: f64, t2169: f64, t2319: f64, t2363: f64, t24969: f64, t24972: f64, t577: f64, t671: f64, t7423: f64, t83979: f64, t83984: f64, t83988: f64, t83991: f64, t83993: f64, t83999: f64, t84001: f64, t84003: f64, t84009: f64, t84012: f64, t84014: f64, t84016: f64, t84018: f64, t85416: f64, t85423: f64, t9416: f64) -> (f64, f64) {
    let t86550 = t85442 + t85585 + t85595 + t86548;
    let t86553 = 0.135e2_f64 * t7423 * t9416 + 81.0_f64 * t85416 * t2319 + 0.405e2_f64 * t24969 * t2363 + t83979 + t83984 + 27.0_f64 * t2169 * t12529 + 0.405e2_f64 * t85423 * t671 + 81.0_f64 * t24972 * t12532 + t83988 + 0.45e1_f64 * t86550 * t577 + t83991 + t83993 + t83999 + t84001 + t84003 + t84009 + t84012 + t84014 + t84016 + t84018;
    (t86550, t86553)
}
