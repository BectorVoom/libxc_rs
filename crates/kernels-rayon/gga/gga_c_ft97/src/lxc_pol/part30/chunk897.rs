//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 897/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk897(t231: f64, t36791: f64, t1100: f64, t17986: f64, t1416: f64, t7447: f64, t1410: f64, t6: f64, t674: f64, t7513: f64, t7639: f64, t797: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t36792 = t36791 * t231;
    let t36796 = t1100 * t17986;
    let t36801 = t7447 * t1416;
    let t36835 = t1410 * t6;
    let t36867 = 1.0_f64 / t7513 / t674;
    let t37041 = 1.0_f64 / t7639 / t797;
    (t36792, t36796, t36801, t36835, t36867, t37041)
}
