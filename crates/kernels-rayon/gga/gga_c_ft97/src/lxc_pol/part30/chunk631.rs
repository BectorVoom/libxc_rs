//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 631/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk631(t6947: f64, t713: f64, t729: f64, t1175: f64, t6061: f64, t242: f64, t27899: f64, t14200: f64, t27763: f64, t14163: f64, t27767: f64, t684: f64, t6861: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t28171 = t729 * t6947 * t713;
    let t28175 = t729 * t1175 * t6061;
    let t28178 = t242 * t27899;
    let t28181 = t14200 * t27763;
    let t28184 = t14163 * t27767;
    let t28187 = t6861 * t684;
    (t28171, t28175, t28178, t28181, t28184, t28187)
}
