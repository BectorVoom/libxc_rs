//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 697/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk697(t28719: f64, t319: f64, t840: f64, t25271: f64, t4176: f64, t15460: f64, t191: f64, t295: f64, t309: f64, t10696: f64, t1501: f64, t4181: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t29120 = t840 * t319 * t28719;
    let t29123 = t25271 * t4176;
    let t29124 = t15460 * t29123;
    let t29127 = t191 * t295;
    let t29128 = t29127 * t309;
    let t29129 = t10696 * t1501;
    let t29130 = t29129 * t4181;
    (t29120, t29123, t29124, t29127, t29128, t29129, t29130)
}
