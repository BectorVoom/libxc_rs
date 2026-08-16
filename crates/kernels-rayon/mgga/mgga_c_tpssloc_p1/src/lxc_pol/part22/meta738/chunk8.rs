//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2430/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2430(t21360: f64, t923: f64, t10756: f64, t10765: f64, t10828: f64, t14263: f64, t14337: f64, t1568: f64, t17443: f64, t17446: f64, t17451: f64, t17499: f64, t17547: f64, t21089: f64, t21207: f64, t21242: f64, t21247: f64, t21306: f64, t2886: f64, t2930: f64, t41826: f64, t42111: f64, t42113: f64, t4433: f64, t4471: f64, t49099: f64, t5775: f64, t60775: f64, t69003: f64, t69005: f64, t933: f64, t950: f64) -> f64 {
    let t69182 = t21360 * t923;
    let t69218 = 1.0_f64 * t69182 * t933 + t69003 - t69005 - 0.12304822629859687989e5_f64 * t41826 * t21242 * t950 + 0.30762056574649219974e4_f64 * t10756 * t17499 * t4471 + 0.91082604192152556044e5_f64 * t42111 * t21089 * t42113 * t950 + 0.96491876992155210402e2_f64 * t10765 * t21306 + 0.96491876992155210402e2_f64 * t2886 * t60775 * t1568 + 0.96491876992155210402e2_f64 * t2886 * t17547 * t4433 + 0.10526802520742363173e2_f64 * t14337 * t17443 - 0.70178683471615754484e1_f64 * t14263 * t17446 - 0.31168546390226634765e3_f64 * t49099 * t17451 - 0.14035736694323150897e2_f64 * t10828 * t21247 * t950 + 0.10526802520742363173e2_f64 * t2930 * t5775 * t4471 + 0.6233709278045326953e3_f64 * t10756 * t21207 * t950;
    t69218
}
