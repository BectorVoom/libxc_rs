//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 968/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk968(t114285: f64, t22633: f64, t28116: f64, t120269: f64, t120276: f64, t120296: f64, t6431: f64, t8466: f64, t1831: f64, t32717: f64, t6427: f64, t31170: f64, t6396: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t127220 = 0.6579736267392905746e-1_f64 * t22633 * t114285 * t28116;
    let t127229 = 0.76763589786250567036e-1_f64 * t120269;
    let t127242 = 0.15352717957250113407e0_f64 * t120276;
    let t127249 = 0.16449340668482264365e-1_f64 * t120296;
    let t127252 = t8466 * t6431;
    let t127254 = t32717 * t1831;
    let t127256 = t8466 * t6427;
    let t127258 = t31170 * t6396;
    (t127220, t127229, t127242, t127249, t127252, t127254, t127256, t127258)
}
