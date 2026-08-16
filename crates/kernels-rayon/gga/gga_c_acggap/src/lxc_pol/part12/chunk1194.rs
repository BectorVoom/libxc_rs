//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1194/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1194(t35379: f64, t35384: f64, t35387: f64, t35390: f64, t35392: f64, t35394: f64, t35396: f64, t35398: f64, t35400: f64, t31164: f64, t31166: f64, t31168: f64, t31179: f64, t31186: f64, t31188: f64, t31193: f64, t31202: f64, t31210: f64) -> f64 {
    let t37519 = 0.62896184579208304138e-3_f64 * t35379;
    let t37522 = 0.61125e-1_f64 * t35384;
    let t37523 = t35387 / 4.0_f64;
    let t37524 = t35390 / 16.0_f64;
    let t37525 = 0.48018900292238105409e-1_f64 * t35392;
    let t37526 = 0.13719685797782315831e-1_f64 * t35394;
    let t37527 = 0.13719685797782315831e-1_f64 * t35396;
    let t37528 = 0.68598428988911579156e-2_f64 * t35398;
    let t37529 = 0.10289764348336736873e-1_f64 * t35400;
    let t37530 = -0.21437009059034868486e-2_f64 * t31164 - 0.10718504529517434243e-2_f64 * t31166 - 0.28582678745379824648e-2_f64 * t31168 - 0.14291339372689912324e-2_f64 * t31179 + 0.12862205435420921092e-2_f64 * t31186 - 0.94344276868812456204e-2_f64 * t31188 + 0.85748036236139473944e-3_f64 * t31193 - t37519 - 0.12579236915841660828e-2_f64 * t31202 + 0.1048269742986805069e-2_f64 * t31210 + t37522 + t37523 + t37524 - t37525 - t37526 + t37527 - t37528 + t37529;
    t37530
}
