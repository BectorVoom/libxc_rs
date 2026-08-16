//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1201/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1201(t7501: f64, t824: f64, t23801: f64, t256: f64, t7758: f64, t805: f64, t243: f64, t2491: f64, t2516: f64, t23543: f64, t23545: f64, t23551: f64, t23553: f64, t23555: f64, t23557: f64, t23561: f64, t23565: f64, t23567: f64, t23569: f64, t23840: f64, t23842: f64, t23846: f64, t23874: f64) -> (f64, f64, f64, f64, f64) {
    let t24792 = t824 * t7501;
    let t24795 = t256 * t23801;
    let t24799 = t805 * t7758;
    let t24804 = t243 / t2516 / t2491;
    let t24824 = -0.23154444444444444445e0_f64 * t23543 - 0.55570666666666666668e0_f64 * t23545 + 0.55570666666666666666e0_f64 * t23551 + 0.12349037037037037037e1_f64 * t23553 + 0.94674375e0_f64 * t23840 - 0.52945875e1_f64 * t23842 + 0.2366859375e0_f64 * t23846 + 0.6311625e0_f64 * t23874 + 0.13892666666666666667e1_f64 * t23555 + 0.166712e1_f64 * t23557 - 0.125034e1_f64 * t23561 - 0.104195e0_f64 * t23565 + 0.27785333333333333333e0_f64 * t23567 + 0.12349037037037037037e0_f64 * t23569;
    (t24792, t24795, t24799, t24804, t24824)
}
