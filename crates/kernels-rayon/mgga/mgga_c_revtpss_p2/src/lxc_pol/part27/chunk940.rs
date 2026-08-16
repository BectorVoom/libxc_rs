//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 940/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk940(t11294: f64, t2927: f64, t287: f64, t2922: f64, t275: f64, t2875: f64, t934: f64, t2926: f64, t11132: f64, t11134: f64, t11136: f64, t11138: f64, t11140: f64, t11147: f64, t11153: f64, t11158: f64, t11162: f64, t11167: f64, t11171: f64) -> (f64, f64, f64, f64) {
    let t11296 = 0.48245938496077605201e2_f64 * t11294 * t2927;
    let t11298 = 1.0_f64 / t2922 / t287;
    let t11299 = t275 * t11298;
    let t11300 = t2875 * t934;
    let t11301 = t11300 * t2926;
    let t11303 = 0.96491876992155210402e2_f64 * t11299 * t11301;
    let t11304 = 28.0_f64 / 27.0_f64 * t11132;
    let t11315 = -t11304 - 4.0_f64 / 9.0_f64 * t11134 + 2.0_f64 / 9.0_f64 * t11136 - 2.0_f64 / 3.0_f64 * t11138 + t11140 / 3.0_f64 - 10.0_f64 / 27.0_f64 * t11147 + 4.0_f64 / 3.0_f64 * t11153 - 2.0_f64 / 3.0_f64 * t11158 - 2.0_f64 * t11162 + 2.0_f64 * t11167 - t11171 / 3.0_f64;
    (t11296, t11300, t11303, t11315)
}
