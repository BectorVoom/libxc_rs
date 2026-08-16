//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 756/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk756(t1257: f64, t1986: f64, t1995: f64, t326: f64, t333: f64, t1265: f64, t2001: f64, t2002: f64, t265: f64, t4789: f64, t638: f64, t71: f64, t7311: f64) -> (f64, f64, f64, f64, f64) {
    let t35535 = t1986 * t1257;
    let t35551 = t1986 * t326 * t1995 * t333;
    let t35554 = t1986 * t1265;
    let t35559 = t2001 * t326 * t2002 * t333;
    let t35565 = t638 * t265 * t4789 * t71 * t7311;
    (t35535, t35551, t35554, t35559, t35565)
}
