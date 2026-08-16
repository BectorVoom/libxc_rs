//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 839/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk839(t16950: f64, t3440: f64, t9115: f64, t16955: f64, t9121: f64, t3439: f64, t12746: f64, t16150: f64, t16011: f64, t3434: f64, t2210: f64, t1882: f64, t4807: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17040 = t3440 * t16950;
    let t17041 = t9115 * t17040;
    let t17044 = t9121 * t16955;
    let t17045 = t3439 * t17044;
    let t17048 = t12746 * t16150;
    let t17049 = t3439 * t17048;
    let t17052 = t3434 * t16011;
    let t17053 = t2210 * t17052;
    let t17056 = t3440 * t16011;
    let t17057 = t3439 * t17056;
    let t17060 = t1882 * t4807;
    (t17041, t17045, t17049, t17053, t17057, t17060)
}
