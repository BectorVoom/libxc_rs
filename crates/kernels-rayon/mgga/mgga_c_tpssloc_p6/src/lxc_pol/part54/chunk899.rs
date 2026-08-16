//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 899/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk899(t225: f64, t4266: f64, t4143: f64, t4145: f64, t1509: f64, t828: f64, t2632: f64, t120: f64, t4233: f64, t1484: f64, t852: f64, t252: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13042 = t4266 * t225;
    let t13053 = t4143 * t225;
    let t13065 = t4145 * t225;
    let t13223 = t1509 * t828;
    let t13228 = t1509 * t2632;
    let t13242 = t120 * t4233;
    let t13351 = t1484 * t828;
    let t13380 = t852 * t1509;
    let t13384 = t252 * t4233;
    (t13042, t13053, t13065, t13223, t13228, t13242, t13351, t13380, t13384)
}
