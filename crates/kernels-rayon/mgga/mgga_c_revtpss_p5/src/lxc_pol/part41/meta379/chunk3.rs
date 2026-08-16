//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1252/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1252(t6152: f64, t945: f64, t15170: f64, t15189: f64, t15312: f64, t15322: f64, t15324: f64, t18944: f64, t18961: f64, t18964: f64, t18967: f64, t18970: f64, t18973: f64) -> (f64, f64) {
    let t19173 = t6152 * t945;
    let t19202 = 0.103295e1_f64 * t18944 + 0.20839e0_f64 * t18961 - 0.69463333333333333334e-1_f64 * t18964 - 0.46308888888888888889e-1_f64 * t18967 - 0.62517e0_f64 * t18970 + 0.41678e0_f64 * t18973 - t15312 + 0.4630888888888888889e-1_f64 * t15170 - 0.45908888888888888888e0_f64 * t15189 + t15322 + t15324;
    (t19173, t19202)
}
