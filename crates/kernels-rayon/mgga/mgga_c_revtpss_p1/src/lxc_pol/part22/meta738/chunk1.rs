//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2800/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2800(t40757: f64, t810: f64, t10732: f64, t10760: f64, t9794: f64, t240: f64, t9731: f64, t2664: f64, t10293: f64, t124: f64, t212: f64, t800: f64) -> (f64, f64, f64, f64, f64) {
    let t40759 = 0.26776076960158126592e-7_f64 * t40757 * t810;
    let t40761 = t10760 * t9794 * t10732;
    let t40763 = t9731 * t240;
    let t40765 = t10760 * t40763 * t2664;
    let t40769 = t800 * t124 * t10293 * t212;
    (t40759, t40761, t40763, t40765, t40769)
}
