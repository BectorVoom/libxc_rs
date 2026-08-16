//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2072/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2072(t1043: f64, t204: f64, t1041: f64, t248: f64, t884: f64, t10337: f64, t964: f64, t340: f64, t625: f64, t221: f64, t339: f64, t344: f64) -> (f64, f64, f64, f64, f64) {
    let t42749 = t204 * t1043;
    let t42752 = t1041 * t248 * t42749 * t884;
    let t42811 = t964 * t10337;
    let t42813 = t625 * t340;
    let t42817 = 0.82304526748971193413e-3_f64 * t339 * t221 * t42813 * t344;
    (t42749, t42752, t42811, t42813, t42817)
}
