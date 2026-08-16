//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1331/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1331(t97525: f64, t97537: f64, t97550: f64, t97565: f64, t13240: f64, t13244: f64, t13247: f64, t1461: f64, t2170: f64, t27102: f64, t4162: f64, t4165: f64, t573: f64, t7696: f64, t95131: f64, t95136: f64, t95140: f64, t95143: f64, t95147: f64, t95149: f64, t95153: f64, t95157: f64, t95160: f64, t95163: f64, t95171: f64, t95173: f64, t95175: f64, param_d: f64) -> (f64, f64) {
    let t97567 = t97525 + t97537 + t97550 + t97565;
    let t97576 = t573 * t97567 * param_d + 6.0_f64 * t13240 * t2170 + 18.0_f64 * t13244 * t2170 + 3.0_f64 * t13247 * t2170 + 9.0_f64 * t1461 * t27102 + 18.0_f64 * t4162 * t7696 + 9.0_f64 * t4165 * t7696 + t95131 + t95136 + t95140 + t95143 + t95147 + t95149 + t95153 + t95157 + t95160 + t95163 + t95171 + t95173 + t95175;
    (t97567, t97576)
}
