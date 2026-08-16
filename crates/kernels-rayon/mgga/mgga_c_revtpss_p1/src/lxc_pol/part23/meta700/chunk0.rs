//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2450/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2450(t1340: f64, t40182: f64, t39821: f64, t40196: f64, t40192: f64, t4038: f64, t9419: f64, t40113: f64, t40169: f64, t3863: f64, t4029: f64, t40135: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t47084 = 0.5848223622634646207e0_f64 * t1340 * t40182;
    let t47086 = 0.61524113149298439947e4_f64 * t1340 * t39821;
    let t47088 = 0.35089341735807877242e1_f64 * t1340 * t40196;
    let t47092 = 0.14035736694323150897e2_f64 * t1340 * t40192;
    let t47093 = t4038 * t9419;
    let t47096 = 0.51947577317044391277e2_f64 * t1340 * t40113;
    let t47098 = 0.91082604192152556044e5_f64 * t1340 * t40169;
    let t47101 = t3863 * t4029;
    let t47109 = 0.6233709278045326953e3_f64 * t1340 * t40135;
    (t47084, t47086, t47088, t47092, t47093, t47096, t47098, t47101, t47109)
}
