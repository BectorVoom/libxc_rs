//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3858/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3858(t47127: f64, t47133: f64, t47135: f64, t48324: f64, t48326: f64, t47145: f64, t47147: f64, t47149: f64, t48331: f64, t48333: f64, t48335: f64, t40076: f64, t40079: f64, t47131: f64, t47138: f64, t47140: f64, t47142: f64, t47152: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t74141 = 0.32530743900905219526e-1_f64 * t47127;
    let t74142 = 0.10843581300301739842e-1_f64 * t47133;
    let t74143 = 0.43374325201206959367e-1_f64 * t47135;
    let t74144 = 0.65061487801810439052e-1_f64 * t48324;
    let t74145 = 32.0_f64 * t48326;
    let t74146 = 0.17315859105681463759e2_f64 * t47145;
    let t74147 = 0.20508037716432813316e4_f64 * t47147;
    let t74148 = 8.0_f64 * t47149;
    let t74149 = 64.0_f64 * t48331;
    let t74150 = 24.0_f64 * t48333;
    let t74151 = 0.2077903092681775651e3_f64 * t48335;
    let t74152 = t74141 + t47131 + t74142 - t74143 - t47138 - t47140 + t47142 - t74144 - t74145 + t40076 - t40079 - t74146 - t74147 - t74148 - t74149 + t47152 - t74150 + t74151;
    (t74141, t74142, t74143, t74144, t74145, t74146, t74147, t74148, t74149, t74150, t74151, t74152)
}
