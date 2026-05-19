//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1194/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1194<F: Float>(t35379: F, t35384: F, t35387: F, t35390: F, t35392: F, t35394: F, t35396: F, t35398: F, t35400: F, t31164: F, t31166: F, t31168: F, t31179: F, t31186: F, t31188: F, t31193: F, t31202: F, t31210: F) -> F {
    let t37519 = F::cast_from(0.62896184579208304138e-3_f64) * t35379;
    let t37522 = F::new(0.61125e-1) * t35384;
    let t37523 = t35387 / F::new(4.0);
    let t37524 = t35390 / F::new(16.0);
    let t37525 = F::cast_from(0.48018900292238105409e-1_f64) * t35392;
    let t37526 = F::cast_from(0.13719685797782315831e-1_f64) * t35394;
    let t37527 = F::cast_from(0.13719685797782315831e-1_f64) * t35396;
    let t37528 = F::cast_from(0.68598428988911579156e-2_f64) * t35398;
    let t37529 = F::cast_from(0.10289764348336736873e-1_f64) * t35400;
    let t37530 = -F::cast_from(0.21437009059034868486e-2_f64) * t31164 - F::cast_from(0.10718504529517434243e-2_f64) * t31166 - F::cast_from(0.28582678745379824648e-2_f64) * t31168 - F::cast_from(0.14291339372689912324e-2_f64) * t31179 + F::cast_from(0.12862205435420921092e-2_f64) * t31186 - F::cast_from(0.94344276868812456204e-2_f64) * t31188 + F::cast_from(0.85748036236139473944e-3_f64) * t31193 - t37519 - F::cast_from(0.12579236915841660828e-2_f64) * t31202 + F::cast_from(0.1048269742986805069e-2_f64) * t31210 + t37522 + t37523 + t37524 - t37525 - t37526 + t37527 - t37528 + t37529;
    t37530
}
