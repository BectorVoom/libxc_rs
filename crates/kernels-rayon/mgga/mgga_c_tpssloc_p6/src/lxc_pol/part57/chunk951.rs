//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 951/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk951(t19451: f64, t8326: f64, t28002: f64, t1458: f64, t7450: f64, t1868: f64, t5493: f64, t1484: f64, t7540: f64, t22960: f64, t25: f64, t28447: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t126118 = 2.0_f64 * t19451 * t8326;
    let t126120 = 4.0_f64 * t28002 * t8326;
    let t126127 = t7450 * t1458;
    let t126132 = t1868 * t5493;
    let t126176 = t1484 * t7540;
    let t126177 = t22960 * t126176;
    let t126180 = t25 * t28447;
    (t126118, t126120, t126127, t126132, t126176, t126177, t126180)
}
