//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 983/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk983(t1588: f64, t3228: f64, t1008: f64, t5232: f64, t1049: f64, t4801: f64, t1483: f64, t3143: f64, t13747: f64, t503: f64, t1068: f64, t1072: f64, t1427: f64, t3114: f64, t3126: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16205 = t3228 * t1588;
    let t16207 = t1008 * t5232;
    let t16209 = t1049 * t4801;
    let t16211 = t3143 * t1483;
    let t16213 = t13747 * t503;
    let t16230 = t1068 * t3114 * t1072 * t1427 * t3126;
    (t16205, t16207, t16209, t16211, t16213, t16230)
}
