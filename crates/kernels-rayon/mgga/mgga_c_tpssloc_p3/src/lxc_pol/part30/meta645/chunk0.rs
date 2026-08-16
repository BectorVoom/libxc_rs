//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2057/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2057(t1611: f64, t23528: f64, t23436: f64, t4640: f64, t14507: f64, t23536: f64, t23540: f64, t23433: f64, t4630: f64, t10189: f64, t1920: f64, t4343: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t88584 = t1611 * t23528;
    let t88591 = t4640 * t23436;
    let t88594 = t14507 * t23536;
    let t88600 = t14507 * t23540;
    let t88604 = t23433 * t4630 / 1152.0_f64;
    let t88622 = t1920 * t10189 * t4343 / 216.0_f64;
    (t88584, t88591, t88594, t88600, t88604, t88622)
}
