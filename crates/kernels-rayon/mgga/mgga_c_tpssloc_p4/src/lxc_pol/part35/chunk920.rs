//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 920/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk920(t135: f64, t6183: f64, t1174: f64, t6177: f64, t248: f64, t3570: f64, t6225: f64, t3506: f64, t11697: f64, t6191: f64, t3577: f64, t6219: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t18329 = t135 * t6183;
    let t18330 = t1174 * t18329;
    let t18332 = t135 * t6177;
    let t18333 = t1174 * t18332;
    let t18356 = t248 * t3570 * t6225;
    let t18357 = t3506 * t18356;
    let t18371 = t11697 * t6191;
    let t18372 = t3577 * t18371;
    let t18375 = t248 * t3570 * t6219;
    (t18329, t18330, t18332, t18333, t18356, t18357, t18371, t18372, t18375)
}
