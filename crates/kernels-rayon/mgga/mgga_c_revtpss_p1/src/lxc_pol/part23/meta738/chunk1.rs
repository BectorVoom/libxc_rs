//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2515/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2515(t51060: f64, t14742: f64, t2689: f64, t243: f64, t9794: f64, t10760: f64, t14495: f64, t14587: f64, t40799: f64, t4372: f64, t9789: f64, t40627: f64, t50451: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t51061 = 35.0_f64 / 24.0_f64 * t51060;
    let t51074 = t2689 * t14742;
    let t51076 = t9794 * t243;
    let t51078 = t10760 * t51076 * t14495;
    let t51079 = 0.13553694749236397037e-4_f64 * t51078;
    let t51081 = t40799 * t51076 * t14587;
    let t51083 = t9789 * t4372;
    let t51086 = t10760 * t40627 * t50451;
    (t51061, t51074, t51079, t51081, t51083, t51086)
}
