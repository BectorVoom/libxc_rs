//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 271/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk271(t1179: f64, t1187: f64, t1188: f64, t1196: f64, t1118: f64, t1124: f64) -> (f64, f64, f64, f64) {
    let t1198 = t1179 * t1187 * t1188;
    let t1200 = 0.5848223622634646207e0_f64 * t1196 * t1198;
    let t1201 = 0.83333333333333333333e-2_f64 * t1118;
    let t1203 = -t1201 + 0.83333333333333333333e-2_f64 * t1124;
    (t1198, t1200, t1201, t1203)
}
