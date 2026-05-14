//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1101/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1101<F: Float>(t10767: F, t154: F, t2048: F, t276: F, t10938: F, t11011: F, t11015: F, t21360: F, t21462: F, t25189: F, t25198: F, t25207: F, t25212: F, t25218: F, t25226: F, t25229: F, t25231: F, t25236: F, t25239: F, t2922: F, t2923: F, t29754: F, t29762: F, t29766: F, t29775: F, t29776: F, t29787: F, t302: F, t735: F, t7586: F) -> (F,) {
    let t29793 = t276 * t154 * t2048 * t10767;
    let t29795 = -0.21437009059034868486e-3 * t2922 * t302 * t29754 * t2923 - t7586 * t10938 / 6.0 + t29762 / 48.0 - 0.28582678745379824648e-3 * t25189 - t21360 + 0.85748036236139473947e-3 * t29766 - t25198 / 72.0 + t25207 / 36.0 - t25212 / 18.0 + t25218 / 144.0 - 0.17149607247227894789e-2 * t25226 - 0.17149607247227894789e-2 * t25229 + 0.91464571985215438875e-2 * t25231 + 0.51448821741683684368e-2 * t21462 * t302 * t29775 * t29776 + 0.57165357490759649297e-3 * t25236 - 0.85748036236139473944e-3 * t25239 + t735 * t11015 / 6.0 - t29787 / 48.0 + t735 * t11011 / 36.0 - t29793 / 288.0;
    (t29795,)
}
