//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 1181/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk1181(t19473: f64, t666: f64, t4043: f64, t4067: f64, t5489: f64, t626: f64, t2331: f64, t5488: f64, t5468: f64, t9384: f64, t659: f64, t1444: f64, t2: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19474 = t19473 * t666;
    let t19477 = t4043 * t4067;
    let t19480 = t626 * t5489;
    let t19482 = t2331 * t5488;
    let t19483 = t19482 * t666;
    let t19488 = t9384 * t5468;
    let t19489 = t19488 * t659;
    let t19492 = t1444 * t2;
    (t19474, t19477, t19480, t19483, t19489, t19492)
}
