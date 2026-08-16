//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1142/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1142(t10035: f64, t10090: f64, t10102: f64, t14120: f64, t14149: f64, t14161: f64, t14166: f64, t14171: f64, t14203: f64, t14221: f64, t1437: f64, t1883: f64, t22316: f64, t22321: f64, t22858: f64, t22863: f64, t22912: f64, t22954: f64, t4114: f64, t5767: f64, t6844: f64, t6862: f64, t6874: f64, t820: f64) -> f64 {
    let t23019 = -0.19756347548806534796e1_f64 * t820 * t5767 * t6844 + 0.19514881078765566038e-2_f64 * t14120 + t10035 - 0.21951497276451705329e-1_f64 * t14149 + 0.34697458558045176417e-2_f64 * t14161 + 0.21951497276451705329e-1_f64 * t14166 - 0.65854491829355115987e0_f64 * t820 * t1437 * t22954 - 0.39512695097613069591e1_f64 * t820 * t10090 * t22858 + 0.39512695097613069591e1_f64 * t820 * t4114 * t22863 - 0.19756347548806534796e1_f64 * t820 * t5767 * t6874 - 0.65854491829355115987e0_f64 * t820 * t1437 * t22912 + 0.58544643236296698113e-1_f64 * t22316 - 0.19514881078765566038e-2_f64 * t14203 - 0.19756347548806534796e1_f64 * t820 * t22321 * t1883 + 0.39512695097613069591e1_f64 * t820 * t14171 * t6862 - 0.34697458558045176417e-2_f64 * t14221 + t10102;
    t23019
}
