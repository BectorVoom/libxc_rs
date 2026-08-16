//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1749/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1749(t1317: f64, t9561: f64, t1340: f64, t40182: f64, t39821: f64, t40196: f64, t9554: f64, t40192: f64, t4038: f64, t9419: f64, t40113: f64, t40169: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t47081 = t1317 * t9561;
    let t47082 = 16.0_f64 * t47081;
    let t47084 = 0.5848223622634646207e0_f64 * t1340 * t40182;
    let t47086 = 0.61524113149298439947e4_f64 * t1340 * t39821;
    let t47088 = 0.35089341735807877242e1_f64 * t1340 * t40196;
    let t47089 = t1317 * t9554;
    let t47090 = 48.0_f64 * t47089;
    let t47092 = 0.14035736694323150897e2_f64 * t1340 * t40192;
    let t47093 = t4038 * t9419;
    let t47094 = 0.4155806185363551302e3_f64 * t47093;
    let t47096 = 0.51947577317044391277e2_f64 * t1340 * t40113;
    let t47098 = 0.91082604192152556044e5_f64 * t1340 * t40169;
    (t47082, t47084, t47086, t47088, t47090, t47092, t47094, t47096, t47098)
}
