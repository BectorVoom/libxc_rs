//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 709/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk709(t23471: f64, t6740: f64, t225: f64, t343: f64, t364: f64, t3034: f64, t371: f64, t1930: f64, t6741: f64, t3030: f64, t3127: f64, t363: f64) -> (f64, f64, f64, f64) {
    let t23472 = t6740 * t23471;
    let t23478 = t343 * t225;
    let t23479 = t23478 * t364;
    let t23508 = 1.0_f64 / t3034 / t371;
    let t23509 = t1930 * t23508;
    let t23510 = t23509 * t6741;
    let t23511 = t3030 * t3127;
    let t23512 = t23511 * t363;
    (t23472, t23479, t23510, t23512)
}
