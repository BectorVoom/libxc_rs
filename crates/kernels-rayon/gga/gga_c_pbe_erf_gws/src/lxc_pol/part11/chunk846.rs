//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 846/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk846(t13347: f64, t2345: f64, t3219: f64, t2118: f64, t3786: f64, t3912: f64, t860: f64, t2255: f64, t3752: f64, t3781: f64, t11564: f64, t3180: f64) -> (f64, f64, f64, f64, f64) {
    let t13349 = t2345 * t3219 * t13347;
    let t13352 = t2118 * t3786;
    let t13353 = t3912 * t13352;
    let t13355 = t13353 * t860 / 32.0_f64;
    let t13357 = t2255 * t3781 * t3752;
    let t13361 = t11564 * t3180 / 16.0_f64;
    (t13349, t13353, t13355, t13357, t13361)
}
