//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1213/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1213<F: Float>(t18002: F, t759: F, t10932: F, t154: F, t18182: F, t276: F, t10767: F, t2048: F, t10938: F, t11011: F, t11015: F, t21360: F, t21462: F, t25189: F, t25198: F, t25207: F, t25212: F, t25218: F, t25226: F, t25229: F, t25231: F, t25236: F, t25239: F, t2922: F, t2923: F, t29754: F, t29762: F, t29766: F, t29775: F, t302: F, t735: F, t7586: F) -> (F, F) {
    let t29776 = t18002 * t759;
    let t29787 = t276 * t154 * t18182 * t10932;
    let t29793 = t276 * t154 * t2048 * t10767;
    let t29795 = -F::cast_from(0.21437009059034868486e-3_f64) * t2922 * t302 * t29754 * t2923 - t7586 * t10938 / F::cast_from(6.0_f64) + t29762 / F::cast_from(48.0_f64) - F::cast_from(0.28582678745379824648e-3_f64) * t25189 - t21360 + F::cast_from(0.85748036236139473947e-3_f64) * t29766 - t25198 / F::cast_from(72.0_f64) + t25207 / F::cast_from(36.0_f64) - t25212 / F::cast_from(18.0_f64) + t25218 / F::cast_from(144.0_f64) - F::cast_from(0.17149607247227894789e-2_f64) * t25226 - F::cast_from(0.17149607247227894789e-2_f64) * t25229 + F::cast_from(0.91464571985215438875e-2_f64) * t25231 + F::cast_from(0.51448821741683684368e-2_f64) * t21462 * t302 * t29775 * t29776 + F::cast_from(0.57165357490759649297e-3_f64) * t25236 - F::cast_from(0.85748036236139473944e-3_f64) * t25239 + t735 * t11015 / F::cast_from(6.0_f64) - t29787 / F::cast_from(48.0_f64) + t735 * t11011 / F::cast_from(36.0_f64) - t29793 / F::cast_from(288.0_f64);
    (t29776, t29795)
}
