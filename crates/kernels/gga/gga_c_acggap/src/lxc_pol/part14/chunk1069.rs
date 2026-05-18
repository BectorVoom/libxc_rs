//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1069/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1069<F: Float>(t30078: F, t30081: F, t30084: F, t30089: F, t30091: F, t30106: F, t30123: F, t30151: F, t33904: F, t33916: F, t36838: F, t38820: F, t38830: F, t38834: F, t38840: F, t38846: F, t38848: F, t38852: F) -> F {
    let t38854 = F::new(0.12862205435420921092e-2) * t38820 - t36838 - t33904 - t30078 - t30081 + t30084 + t30089 + F::new(0.21437009059034868486e-3) * t30091 - F::new(0.62896184579208304136e-3) * t33916 + F::new(0.18868855373762491241e-2) * t30106 + F::new(0.42874018118069736972e-3) * t30123 - F::new(0.62896184579208304134e-3) * t30151 - F::new(0.21437009059034868486e-3) * t38830 - F::new(0.10718504529517434243e-3) * t38834 - F::new(0.64311027177104605458e-3) * t38840 + F::new(0.47172138434406228102e-2) * t38846 - F::new(0.68598428988911579156e-2) * t38848 - F::new(0.15724046144802076034e-2) * t38852;
    t38854
}
