//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1164/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1164(t34127: f64, t34130: f64, t34156: f64, t34158: f64, t30301: f64, t34123: f64, t34132: f64, t34135: f64, t34138: f64, t34142: f64, t34146: f64, t34148: f64, t34150: f64, t34152: f64, t34154: f64, t34162: f64, t34166: f64) -> f64 {
    let t36950 = 0.28582678745379824648e-3_f64 * t34127;
    let t36951 = 0.42874018118069736972e-3_f64 * t34130;
    let t36961 = 0.18868855373762491241e-2_f64 * t34156;
    let t36962 = 0.13719685797782315831e-1_f64 * t34158;
    let t36966 = -0.42874018118069736972e-3_f64 * t34123 + t36950 + t36951 - 0.75475421495049964965e-2_f64 * t34132 + 0.21437009059034868486e-2_f64 * t34135 + 0.85748036236139473944e-3_f64 * t34138 + 0.31448092289604152069e-2_f64 * t34142 + 0.62896184579208304138e-2_f64 * t34146 + 0.68598428988911579156e-2_f64 * t34148 - 0.34299214494455789578e-2_f64 * t34150 + 0.34299214494455789578e-2_f64 * t34152 - 0.17149607247227894789e-2_f64 * t34154 - t36961 - t36962 - 0.7717323261252552655e-1_f64 * t34162 + 0.64311027177104605458e-2_f64 * t34166 + 0.40015750243531754507e-2_f64 * t30301;
    t36966
}
