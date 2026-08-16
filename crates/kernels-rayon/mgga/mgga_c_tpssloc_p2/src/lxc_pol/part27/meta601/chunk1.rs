//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2070/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2070(t23337: f64, t82431: f64, t10336: f64, t1920: f64, t1922: f64, t23391: f64, t6680: f64, t3173: f64, t3175: f64, t1921: f64, t1054: f64, t3206: f64) -> (f64, f64, f64, f64, f64) {
    let t82432 = t82431 * t23337;
    let t82436 = 0.30461741978670859935e-2_f64 * t1920 * t10336 * t1922;
    let t82437 = t6680 * t23391;
    let t82441 = t3173 * t3175;
    let t82442 = t1921 * t82441;
    let t82457 = t1054 * t3206;
    (t82432, t82436, t82437, t82442, t82457)
}
