//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 637/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk637(t128: f64, t39: f64, t2035: f64, t1995: f64, t2031: f64, t554: f64, t7883: f64, t1701: f64, t2058: f64, t6: f64, t133: f64, t1702: f64, t2059: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8811 = t128 * t39;
    let t8812 = t8811 * t2035;
    let t8825 = t1995 * t2031;
    let t8828 = t7883 * t554;
    let t8829 = t1701 * t8828;
    let t8832 = t2058 * t6;
    let t8833 = t133 * t8832;
    let t8835 = t1701 * t1702 * t2059;
    (t8811, t8812, t8825, t8829, t8832, t8833, t8835)
}
