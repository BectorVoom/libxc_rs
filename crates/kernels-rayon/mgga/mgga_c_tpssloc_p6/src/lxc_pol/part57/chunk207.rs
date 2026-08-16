//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 207/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk207(t1039: f64, t364: f64, t354: f64, t270: f64, t283: f64, t61: f64, t225: f64, t382: f64) -> (f64, f64, f64, f64, f64) {
    let t1040 = t364 * t1039;
    let t1041 = t354 * t1040;
    let t1043 = 1.0_f64 / t283 / t270;
    let t1044 = t61 * t1043;
    let t1052 = t382 * t225;
    (t1040, t1041, t1043, t1044, t1052)
}
