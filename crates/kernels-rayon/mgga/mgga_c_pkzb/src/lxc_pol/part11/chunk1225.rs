//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1225/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1225(t10771: f64, t237: f64, t732: f64, t3604: f64, t721: f64, t1108: f64, t20671: f64, t7411: f64, t9232: f64, t20896: f64, t9236: f64, t29753: f64, t30193: f64, t30195: f64, t30197: f64, t30200: f64, t30203: f64, t30205: f64, t30208: f64, t30211: f64, t30213: f64, t30216: f64, t30219: f64, t30221: f64, t30223: f64, t30225: f64, t30227: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t30228 = t237 * t10771;
    let t30230 = 0.5848223622634646207e0_f64 * t30228 * t732;
    let t30231 = t3604 * t721;
    let t30234 = 0.10526802520742363173e2_f64 * t20671 * t1108 * t30231;
    let t30236 = 0.96491876992155210402e2_f64 * t7411 * t9232;
    let t30238 = 0.1551780387578202009e4_f64 * t20896 * t9236;
    let t30239 = -t29753 - t30193 + t30195 - t30197 + t30200 + t30203 - t30205 - t30208 - t30211 + t30213 + t30216 + t30219 - t30221 + t30223 - t30225 + t30227 - t30230 - t30234 + t30236 + t30238;
    (t30230, t30231, t30234, t30236, t30238, t30239)
}
