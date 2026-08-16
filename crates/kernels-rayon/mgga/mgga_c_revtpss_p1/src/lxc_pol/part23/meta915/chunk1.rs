//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2950/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2950(t11385: f64, t23467: f64, t934: f64, t11299: f64, t4631: f64, t6145: f64, t23550: f64, t41588: f64, t23547: f64, t2874: f64, t23546: f64, t2926: f64) -> (f64, f64, f64, f64, f64) {
    let t78319 = 0.57895126195293126241e3_f64 * t11385 * t23467 * t934;
    let t78322 = 0.28947563097646563121e3_f64 * t11299 * t6145 * t4631;
    let t78325 = 0.62071215503128080361e4_f64 * t41588 * t23550 * t934;
    let t78328 = 2.0_f64 * t2874 * t23547 * t934;
    let t78329 = t23546 * t2926;
    (t78319, t78322, t78325, t78328, t78329)
}
