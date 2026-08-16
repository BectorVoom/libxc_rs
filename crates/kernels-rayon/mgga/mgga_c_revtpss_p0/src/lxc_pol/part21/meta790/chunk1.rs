//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2848/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2848(t11341: f64, t141: f64, t51998: f64, t15129: f64, t2251: f64, t930: f64, t41361: f64, t41363: f64, t41369: f64, t51978: f64, t51981: f64, t51984: f64, t51987: f64, t51990: f64, t51995: f64) -> (f64, f64, f64, f64) {
    let t52000 = t141 * t11341 * t51998;
    let t52002 = t15129 * t2251;
    let t52004 = t141 * t930 * t52002;
    let t52009 = 0.31310740740740740741e0_f64 * t51978 - 0.8585111111111111111e-1_f64 * t51981 + 0.49671e0_f64 * t51984 + 0.16557e0_f64 * t51987 + 0.49671e0_f64 * t51990 + 0.49671e0_f64 * t51995 + 0.44152e0_f64 * t52000 - 0.149013e1_f64 * t52004 + 0.93932222222222222223e0_f64 * t41361 + 0.80513333333333333335e0_f64 * t41363 - 0.40256666666666666668e0_f64 * t41369;
    (t52000, t52002, t52004, t52009)
}
