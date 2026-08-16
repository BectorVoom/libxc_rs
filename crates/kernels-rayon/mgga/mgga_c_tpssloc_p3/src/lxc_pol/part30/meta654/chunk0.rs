//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2070/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2070(t1920: f64, t2966: f64, t7561: f64, t225: f64, t25789: f64, t23384: f64, t25802: f64, t23587: f64, t7560: f64, t25410: f64, t25798: f64, t25822: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t89617 = t1920 * t2966 * t7561;
    let t89620 = t25789 * t225;
    let t89630 = 0.18277045187202515961e-2_f64 * t23384 * t25802;
    let t89648 = t7560 * t23587;
    let t89653 = 0.54831135561607547884e-2_f64 * t23384 * t25410;
    let t89662 = 0.54831135561607547884e-2_f64 * t23384 * t25798;
    let t89666 = t25822 * t225;
    (t89617, t89620, t89630, t89648, t89653, t89662, t89666)
}
