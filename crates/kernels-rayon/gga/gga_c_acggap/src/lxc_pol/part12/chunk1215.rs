//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1215/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1215(t36151: f64, t36156: f64, t36162: f64, t36175: f64, t36177: f64, t31759: f64, t31761: f64, t31763: f64, t31774: f64, t31782: f64, t31790: f64, t36147: f64, t36149: f64, t36160: f64, t36165: f64, t36169: f64, t36173: f64, t36181: f64) -> f64 {
    let t37888 = 7.0_f64 / 72.0_f64 * t36151;
    let t37892 = 0.12579236915841660828e-2_f64 * t36156;
    let t37894 = 0.85748036236139473944e-3_f64 * t36162;
    let t37898 = 0.18868855373762491241e-2_f64 * t36175;
    let t37899 = 0.68598428988911579156e-2_f64 * t36177;
    let t37901 = -0.7145669686344956162e-3_f64 * t31759 - 0.85748036236139473944e-3_f64 * t31761 - 0.42874018118069736972e-3_f64 * t31763 + t36147 / 8.0_f64 + t36149 / 24.0_f64 + t37888 + 0.3361875e0_f64 * t31774 + 0.16809375e0_f64 * t31782 - 0.1120625e0_f64 * t31790 - t37892 - 0.62896184579208304138e-3_f64 * t36160 + t37894 + 0.85748036236139473944e-3_f64 * t36165 + 0.85748036236139473944e-3_f64 * t36169 + 0.42874018118069736972e-3_f64 * t36173 - t37898 - t37899 - 0.18868855373762491241e-2_f64 * t36181;
    t37901
}
