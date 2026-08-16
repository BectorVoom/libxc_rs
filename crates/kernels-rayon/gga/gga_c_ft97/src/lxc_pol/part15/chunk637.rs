//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 637/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk637(t1882: f64, t4617: f64, t4574: f64, t4565: f64, t4561: f64, t4557: f64, t15606: f64, t15609: f64, t15612: f64, t15891: f64, t15894: f64, t15899: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t16255 = t1882 * t4617;
    let t16296 = t1882 * t4574;
    let t16298 = t1882 * t4565;
    let t16300 = t1882 * t4561;
    let t16302 = t1882 * t4557;
    let t16336 = 2.0_f64 / 27.0_f64 * t15606;
    let t16337 = 2.0_f64 / 9.0_f64 * t15609;
    let t16338 = t15612 / 9.0_f64;
    let t16342 = t15891 / 3.0_f64;
    let t16343 = 2.0_f64 / 3.0_f64 * t15894;
    let t16346 = 2.0_f64 / 9.0_f64 * t15899;
    (t16255, t16296, t16298, t16300, t16302, t16336, t16337, t16338, t16342, t16343, t16346)
}
