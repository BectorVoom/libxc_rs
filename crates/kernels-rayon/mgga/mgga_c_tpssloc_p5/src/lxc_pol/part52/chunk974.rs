//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 974/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk974(t225: f64, t24600: f64, t1089: f64, t1240: f64, t3597: f64, t1235: f64, t7284: f64, t1251: f64, t2122: f64, t1170: f64, t7295: f64, t2121: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t24601 = t24600 * t225;
    let t24602 = t1240 * t1089;
    let t24615 = t225 * t3597;
    let t24633 = t7284 * t1235;
    let t24637 = t1240 * t1251;
    let t24638 = t2122 * t24637;
    let t24645 = t1170 * t7295;
    let t24646 = t2121 * t24645;
    (t24601, t24602, t24615, t24633, t24638, t24646)
}
