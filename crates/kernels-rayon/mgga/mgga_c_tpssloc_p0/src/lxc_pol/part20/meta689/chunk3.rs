//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2614/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2614(t1227: f64, t14706: f64, t248: f64, t3521: f64, t11814: f64, t4997: f64, t15492: f64, t3536: f64, t11781: f64, t15594: f64, t1748: f64, t3531: f64, t3578: f64, t44918: f64, t45015: f64, t45020: f64, t45027: f64, t45044: f64, t5005: f64, t52236: f64, t52893: f64) -> f64 {
    let t53114 = t1227 * t248 * t3521 * t14706;
    let t53116 = t11814 * t4997;
    let t53118 = t3536 * t15492;
    let t53129 = -t45015 / 1152.0_f64 + t45020 / 3456.0_f64 - 5.0_f64 / 5184.0_f64 * t5005 * t11781 - t53114 / 2304.0_f64 + t53116 / 1536.0_f64 + t53118 / 768.0_f64 - t44918 * t1748 / 4608.0_f64 - t15594 * t3531 / 768.0_f64 - t45027 / 1152.0_f64 - t52893 * t3578 * t52236 / 256.0_f64 - 5.0_f64 / 1296.0_f64 * t45044;
    t53129
}
