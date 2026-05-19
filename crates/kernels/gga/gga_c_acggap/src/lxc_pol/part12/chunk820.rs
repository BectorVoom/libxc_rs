//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 820/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk820<F: Float>(t8507: F, t8509: F, t8527: F, t8531: F, t8533: F, t8546: F, t8503: F, t8512: F, t8516: F, t8519: F, t8523: F, t8529: F, t8537: F, t8542: F, t8550: F) -> F {
    let t9190 = F::cast_from(0.28582678745379824648e-3_f64) * t8507;
    let t9191 = F::cast_from(0.15724046144802076034e-2_f64) * t8509;
    let t9196 = F::cast_from(0.14291339372689912324e-3_f64) * t8527;
    let t9198 = F::cast_from(0.12862205435420921092e-2_f64) * t8531;
    let t9199 = F::cast_from(0.10718504529517434243e-2_f64) * t8533;
    let t9202 = F::cast_from(0.7145669686344956162e-3_f64) * t8546;
    let t9204 = -F::cast_from(0.42874018118069736972e-3_f64) * t8503 - t9190 + t9191 + F::cast_from(0.17149607247227894789e-2_f64) * t8512 + F::cast_from(0.21437009059034868486e-2_f64) * t8516 - F::cast_from(0.21437009059034868486e-3_f64) * t8519 - F::cast_from(0.21437009059034868486e-3_f64) * t8523 - t9196 + F::cast_from(0.31448092289604152069e-3_f64) * t8529 + t9198 - t9199 - F::cast_from(0.10718504529517434243e-2_f64) * t8537 - F::cast_from(0.10718504529517434243e-2_f64) * t8542 - t9202 + F::cast_from(0.15724046144802076034e-2_f64) * t8550;
    t9204
}
