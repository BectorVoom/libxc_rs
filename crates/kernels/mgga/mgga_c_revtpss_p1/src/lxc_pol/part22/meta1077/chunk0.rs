//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3858/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3858<F: Float>(t47127: F, t47133: F, t47135: F, t48324: F, t48326: F, t47145: F, t47147: F, t47149: F, t48331: F, t48333: F, t48335: F, t40076: F, t40079: F, t47131: F, t47138: F, t47140: F, t47142: F, t47152: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t74141 = F::cast_from(0.32530743900905219526e-1_f64) * t47127;
    let t74142 = F::cast_from(0.10843581300301739842e-1_f64) * t47133;
    let t74143 = F::cast_from(0.43374325201206959367e-1_f64) * t47135;
    let t74144 = F::cast_from(0.65061487801810439052e-1_f64) * t48324;
    let t74145 = F::cast_from(32.0_f64) * t48326;
    let t74146 = F::cast_from(0.17315859105681463759e2_f64) * t47145;
    let t74147 = F::cast_from(0.20508037716432813316e4_f64) * t47147;
    let t74148 = F::cast_from(8.0_f64) * t47149;
    let t74149 = F::cast_from(64.0_f64) * t48331;
    let t74150 = F::cast_from(24.0_f64) * t48333;
    let t74151 = F::cast_from(0.2077903092681775651e3_f64) * t48335;
    let t74152 = t74141 + t47131 + t74142 - t74143 - t47138 - t47140 + t47142 - t74144 - t74145 + t40076 - t40079 - t74146 - t74147 - t74148 - t74149 + t47152 - t74150 + t74151;
    (t74141, t74142, t74143, t74144, t74145, t74146, t74147, t74148, t74149, t74150, t74151, t74152)
}
