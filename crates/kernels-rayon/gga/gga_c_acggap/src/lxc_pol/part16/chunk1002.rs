//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1002/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1002(t35410: f64, t435: f64, t7815: f64, t2299: f64, t7780: f64, t7637: f64, t8545: f64, t1429: f64, t7614: f64, t1413: f64, t7685: f64, t1441: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t35411 = t35410 / 96.0_f64;
    let t35413 = t7815 * t435;
    let t35418 = t7780 * t2299;
    let t35425 = t7637 * t8545;
    let t35436 = t7614 * t1429;
    let t35447 = t7685 * t1413;
    let t35448 = 0.40015750243531754508e-2_f64 * t35447;
    let t35451 = t7614 * t1441;
    (t35411, t35413, t35418, t35425, t35436, t35448, t35451)
}
