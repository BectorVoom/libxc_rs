//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1029/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1029(t1181: f64, t4822: f64, t599: f64, t8463: f64, t30301: f64, t34123: f64, t34127: f64, t34131: f64, t34133: f64, t34135: f64, t34138: f64, t34142: f64, t34146: f64, t34148: f64, t34150: f64, t34152: f64, t34154: f64, t34156: f64, t34159: f64, t34162: f64) -> f64 {
    let t34166 = t8463 * t1181 * t599 * t4822;
    let t34169 = -0.21437009059034868486e-3_f64 * t34123 + 0.14291339372689912324e-3_f64 * t34127 + t34131 - t34133 + 0.10718504529517434243e-2_f64 * t34135 + 0.42874018118069736972e-3_f64 * t34138 + 0.15724046144802076034e-2_f64 * t34142 + 0.31448092289604152068e-2_f64 * t34146 + 0.34299214494455789578e-2_f64 * t34148 - 0.17149607247227894789e-2_f64 * t34150 + 0.17149607247227894789e-2_f64 * t34152 - 0.85748036236139473944e-3_f64 * t34154 - 0.94344276868812456204e-3_f64 * t34156 - t34159 - 0.38586616306262763274e-1_f64 * t34162 + 0.32155513588552302729e-2_f64 * t34166 + 0.20007875121765877254e-2_f64 * t30301;
    t34169
}
