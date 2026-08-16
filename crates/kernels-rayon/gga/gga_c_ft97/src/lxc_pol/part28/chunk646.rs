//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 646/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk646(t1882: f64, t6484: f64, t11552: f64, t25929: f64, t6480: f64, t6471: f64, t379: f64, t6478: f64, t8557: f64, t11863: f64, t25933: f64, t25919: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t26295 = t1882 * t6484;
    let t26297 = t11552 * t25929;
    let t26301 = t1882 * t6480;
    let t26303 = t1882 * t6471;
    let t26305 = t6478 * t379;
    let t26306 = t8557 * t26305;
    let t26309 = t11863 * t25933;
    let t26312 = t11863 * t25919;
    (t26295, t26297, t26301, t26303, t26305, t26306, t26309, t26312)
}
