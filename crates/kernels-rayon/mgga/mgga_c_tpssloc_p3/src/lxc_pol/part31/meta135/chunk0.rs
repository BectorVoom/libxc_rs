//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 711/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk711(t1932: f64, t3508: f64, t1209: f64, t3032: f64, t3499: f64, t475: f64, t500: f64, t526: f64, t528: f64, t118: f64, t521: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3612 = t1932 * t3508;
    let t3623 = t3032 * t1209;
    let t3624 = t3499 * t3623;
    let t3625 = t1932 * t475;
    let t3639 = t500 * t500;
    let t3640 = 1.0_f64 / t3639;
    let t3664 = 1.0_f64 / t526;
    let t3672 = 1.0_f64 / t528;
    let t3684 = t521 * t118;
    (t3612, t3623, t3624, t3625, t3639, t3640, t3664, t3672, t3684)
}
