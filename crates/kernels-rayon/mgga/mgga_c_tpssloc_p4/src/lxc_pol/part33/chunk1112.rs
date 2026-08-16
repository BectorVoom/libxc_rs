//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1112/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1112(t1926: f64, t3158: f64, t1942: f64, t3082: f64, t344: f64, t40: f64, t1009: f64, t6740: f64, t225: f64, t343: f64, t364: f64, t3034: f64, t371: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t23447 = t1926 * t3158 / 432.0_f64;
    let t23469 = t1942 * t3082 / 6912.0_f64;
    let t23470 = t40 * t344;
    let t23471 = t23470 * t1009;
    let t23472 = t6740 * t23471;
    let t23478 = t343 * t225;
    let t23479 = t23478 * t364;
    let t23508 = 1.0_f64 / t3034 / t371;
    (t23447, t23469, t23470, t23472, t23479, t23508)
}
