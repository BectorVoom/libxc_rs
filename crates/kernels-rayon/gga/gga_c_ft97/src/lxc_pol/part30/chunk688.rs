//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 688/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk688(t681: f64, t7027: f64, t6971: f64, t317: f64, t4129: f64, t6222: f64, t193: f64, t28835: f64, t6223: f64, t24964: f64, t6970: f64, t1501: f64, t15133: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t28960 = t681 * t7027;
    let t28963 = t681 * t6971;
    let t28966 = t317 * t4129;
    let t28967 = t6222 * t28966;
    let t28968 = t193 * t28967;
    let t28971 = t28835 * t6223;
    let t28972 = t193 * t28971;
    let t28977 = t24964 * t6970;
    let t28978 = t193 * t28977;
    let t28983 = t15133 * t1501;
    (t28960, t28963, t28966, t28968, t28972, t28978, t28983)
}
