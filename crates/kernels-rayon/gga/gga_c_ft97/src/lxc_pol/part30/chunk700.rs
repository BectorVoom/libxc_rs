//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 700/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk700(t1255: f64, t6260: f64, t840: f64, t7131: f64, t824: f64, t1508: f64, t4129: f64, t1212: f64, t6393: f64, t684: f64, t7105: f64, t10703: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t29170 = t840 * t1255 * t6260;
    let t29174 = t840 * t7131 * t824;
    let t29178 = t840 * t1508 * t4129;
    let t29182 = t840 * t6393 * t1212;
    let t29185 = t7105 * t684;
    let t29186 = t10703 * t29185;
    (t29170, t29174, t29178, t29182, t29185, t29186)
}
