//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2436/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2436(t12025: f64, t3127: f64, t3172: f64, t3105: f64, t3196: f64, t11643: f64, t11656: f64, t11648: f64, t3124: f64, t1041: f64, t11622: f64, t12021: f64, t3173: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t42193 = t3127 * t3172 * t12025;
    let t42195 = t3196 * t3105;
    let t42204 = t11656 * t11643;
    let t42227 = t3124 * t11648;
    let t42230 = t1041 * t3172 * t11622;
    let t42232 = t12021 * t3173;
    (t42193, t42195, t42204, t42227, t42230, t42232)
}
