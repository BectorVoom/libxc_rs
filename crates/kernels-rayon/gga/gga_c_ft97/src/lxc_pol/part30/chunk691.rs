//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 691/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk691(t1212: f64, t880: f64, t6222: f64, t193: f64, t28719: f64, t798: f64, t317: f64, t4246: f64, t6386: f64, t7114: f64, t875: f64, t24898: f64, t4176: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t29033 = t880 * t1212;
    let t29034 = t6222 * t29033;
    let t29035 = t193 * t29034;
    let t29040 = t798 * t28719;
    let t29041 = t29040 * t317;
    let t29042 = t193 * t29041;
    let t29045 = t4246 * t6386;
    let t29047 = t7114 * t875;
    let t29051 = t24898 * t4176;
    (t29033, t29035, t29040, t29042, t29045, t29047, t29051)
}
