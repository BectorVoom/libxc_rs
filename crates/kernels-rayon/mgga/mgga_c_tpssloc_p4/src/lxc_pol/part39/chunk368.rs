//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 368/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk368(t1086: f64, t1111: f64, t1092: f64, t1103: f64, t1108: f64, t1115: f64) -> (f64, f64, f64) {
    let t1150 = 0.301925e0_f64 * t1086;
    let t1153 = 0.82785e-1_f64 * t1111;
    let t1155 = 0.258925e1_f64 * t1103 - t1150 + 0.301925e0_f64 * t1092 + 0.16504875e0_f64 * t1108 - t1153 + 0.82785e-1_f64 * t1115;
    (t1150, t1153, t1155)
}
