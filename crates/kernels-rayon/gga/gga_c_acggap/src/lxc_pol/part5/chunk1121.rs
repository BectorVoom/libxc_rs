//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1121/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1121(t6021: f64, t691: f64, t288: f64, t5474: f64, t75: f64, t682: f64, t11945: f64, t1708: f64, t4: f64, t657: f64, t12157: f64, t12665: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t20042 = t6021 * t691;
    let t20043 = 0.17315859105681463759e2_f64 * t20042;
    let t20045 = t5474 * t75 * t288;
    let t20046 = 0.11696447245269292414e1_f64 * t20045;
    let t20047 = t6021 * t682;
    let t20048 = 0.5848223622634646207e0_f64 * t20047;
    let t20049 = 0.20508037716432813316e4_f64 * t11945;
    let t20051 = t1708 * t4 * t657;
    let t20052 = 0.10843581300301739842e-1_f64 * t20051;
    let t20053 = 24.0_f64 * t12157;
    let t20054 = 0.65061487801810439052e-1_f64 * t12665;
    (t20043, t20046, t20048, t20049, t20052, t20053, t20054)
}
