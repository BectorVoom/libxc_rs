//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 361/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk361(t1128: f64, t1153: f64, t1156: f64, t1161: f64, t1170: f64, t1176: f64, t1180: f64, t1189: f64, t300: f64, t435: f64, t439: f64) -> (f64, f64, f64) {
    let t1193 = t300 * (-0.310907e-1_f64 * t1156 * t435 + 1.0_f64 * t1161 * t1170 + t1128 - t1153 - 0.19751673498613801407e-1_f64 * t1176 + 0.5848223622634646207e0_f64 * t1180 * t1189);
    let t1195 = 0.19751673498613801407e-1_f64 * t300 * t1176;
    let t1196 = t300 * t439;
    (t1193, t1195, t1196)
}
