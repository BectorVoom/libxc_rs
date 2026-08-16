//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3295/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3295(t61519: f64, t62429: f64, t62431: f64, t62435: f64, t62439: f64, t62441: f64, t62443: f64, t62445: f64, t62453: f64, t62458: f64, t62460: f64, t828: f64, t851: f64, t855: f64) -> f64 {
    let t62462 = -0.28582678745379824648e-3_f64 * t62429 - 0.27104001498285508387e-2_f64 * t62431 + 0.17149607247227894789e-2_f64 * t62435 - 0.57165357490759649296e-3_f64 * t62439 + 0.80031500487063509014e-2_f64 * t62441 + 0.15244095330869239812e-3_f64 * t62443 - 0.76220476654346199061e-4_f64 * t62445 - 0.85748036236139473944e-3_f64 * t851 * t855 * t828 * t61519 - 0.10164000561857065645e-3_f64 * t62453 + 0.14291339372689912324e-4_f64 * t62458 + 0.32012600194825403606e-1_f64 * t62460;
    t62462
}
