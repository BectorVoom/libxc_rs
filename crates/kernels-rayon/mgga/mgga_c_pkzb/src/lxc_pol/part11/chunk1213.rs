//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1213/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1213(t18002: f64, t759: f64, t10932: f64, t154: f64, t18182: f64, t276: f64, t10767: f64, t2048: f64, t10938: f64, t11011: f64, t11015: f64, t21360: f64, t21462: f64, t25189: f64, t25198: f64, t25207: f64, t25212: f64, t25218: f64, t25226: f64, t25229: f64, t25231: f64, t25236: f64, t25239: f64, t2922: f64, t2923: f64, t29754: f64, t29762: f64, t29766: f64, t29775: f64, t302: f64, t735: f64, t7586: f64) -> (f64, f64) {
    let t29776 = t18002 * t759;
    let t29787 = t276 * t154 * t18182 * t10932;
    let t29793 = t276 * t154 * t2048 * t10767;
    let t29795 = -0.21437009059034868486e-3_f64 * t2922 * t302 * t29754 * t2923 - t7586 * t10938 / 6.0_f64 + t29762 / 48.0_f64 - 0.28582678745379824648e-3_f64 * t25189 - t21360 + 0.85748036236139473947e-3_f64 * t29766 - t25198 / 72.0_f64 + t25207 / 36.0_f64 - t25212 / 18.0_f64 + t25218 / 144.0_f64 - 0.17149607247227894789e-2_f64 * t25226 - 0.17149607247227894789e-2_f64 * t25229 + 0.91464571985215438875e-2_f64 * t25231 + 0.51448821741683684368e-2_f64 * t21462 * t302 * t29775 * t29776 + 0.57165357490759649297e-3_f64 * t25236 - 0.85748036236139473944e-3_f64 * t25239 + t735 * t11015 / 6.0_f64 - t29787 / 48.0_f64 + t735 * t11011 / 36.0_f64 - t29793 / 288.0_f64;
    (t29776, t29795)
}
