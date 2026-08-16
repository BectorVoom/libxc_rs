//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1069/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1069(t30078: f64, t30081: f64, t30084: f64, t30089: f64, t30091: f64, t30106: f64, t30123: f64, t30151: f64, t33904: f64, t33916: f64, t36838: f64, t38820: f64, t38830: f64, t38834: f64, t38840: f64, t38846: f64, t38848: f64, t38852: f64) -> f64 {
    let t38854 = 0.12862205435420921092e-2_f64 * t38820 - t36838 - t33904 - t30078 - t30081 + t30084 + t30089 + 0.21437009059034868486e-3_f64 * t30091 - 0.62896184579208304136e-3_f64 * t33916 + 0.18868855373762491241e-2_f64 * t30106 + 0.42874018118069736972e-3_f64 * t30123 - 0.62896184579208304134e-3_f64 * t30151 - 0.21437009059034868486e-3_f64 * t38830 - 0.10718504529517434243e-3_f64 * t38834 - 0.64311027177104605458e-3_f64 * t38840 + 0.47172138434406228102e-2_f64 * t38846 - 0.68598428988911579156e-2_f64 * t38848 - 0.15724046144802076034e-2_f64 * t38852;
    t38854
}
