//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 1324/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk1324(t224: f64, t37496: f64, t37642: f64, t38080: f64, t38515: f64, t12337: f64, t12335: f64, t12330: f64, t12347: f64, t12575: f64, t12574: f64, t987: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t38518 = t224 * (t37496 + t37642 + t38080 + t38515);
    let t38525 = 4.0_f64 * t12337;
    let t38526 = 4.0_f64 * t12335;
    let t38527 = 2.0_f64 * t12330;
    let t38528 = 4.0_f64 * t12347;
    let t38530 = 2.0_f64 * t12575;
    let t38858 = t987 * t12574;
    (t38518, t38525, t38526, t38527, t38528, t38530, t38858)
}
