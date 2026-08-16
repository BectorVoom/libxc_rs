//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1278/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1278(t15134: f64, t51563: f64, t1161: f64, t274: f64, t1123: f64, t1178: f64, t13917: f64, t2416: f64, t938: f64, t11525: f64, t51066: f64, t53865: f64, param_a_c: f64) -> (f64, f64, f64, f64) {
    let t56242 = t51563 * t15134;
    let t56246 = t274 * t1161;
    let t56250 = t13917 * t1178 * t2416 * param_a_c * t1123 * t56246 * t938;
    let t56255 = t53865 * t51066 * t11525;
    (t56242, t56246, t56250, t56255)
}
