//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 581/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk581(t3928: f64, t945: f64, t1167: f64, t2053: f64, t38: f64, t531: f64, t1477: f64, t2060: f64, t279: f64, t2059: f64, t116: f64, t784: f64, t799: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3929 = t3928 * t945;
    let t3931 = t1167 * t1167;
    let t3932 = t3931 * t2053;
    let t4258 = t38 * t531;
    let t4259 = 1.0_f64 / t4258;
    let t4339 = t2060 * t1477 * t279;
    let t4340 = t2059 * t4339;
    let t4341 = 0.31636214830824236053e1_f64 * t4340;
    let t4347 = t799 * t784 * t116;
    (t3929, t3931, t3932, t4258, t4259, t4339, t4340, t4341, t4347)
}
