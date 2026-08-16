//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 555/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk555(t1227: f64, t3523: f64, t1009: f64, t1190: f64, t1011: f64, t1212: f64, t374: f64, t486: f64, t677: f64, t485: f64, t1203: f64, t1222: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3524 = t1227 * t3523;
    let t3534 = t1190 * t1009;
    let t3535 = t3534 * t1011;
    let t3536 = t3535 * t1212;
    let t3540 = t374 * t677 * t486;
    let t3542 = t485 * t3540 / 13824.0_f64;
    let t3543 = t1203 * t1222;
    (t3524, t3534, t3535, t3536, t3540, t3542, t3543)
}
