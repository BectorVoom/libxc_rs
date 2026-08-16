//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1360/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1360(t1118: f64, t1133: f64, t361: f64, t3223: f64, t50998: f64, t12239: f64, t14121: f64, t11409: f64, t3965: f64, t12250: f64, t3959: f64, t2409: f64, t35433: f64) -> (f64, f64, f64, f64, f64) {
    let t57384 = t361 * t1118 * t1133;
    let t57386 = t50998 * t57384 * t3223;
    let t57390 = t14121 * t12239;
    let t57393 = t3965 * t11409;
    let t57395 = t3959 * t12250;
    let t57398 = t3959 * t2409 * t35433;
    (t57386, t57390, t57393, t57395, t57398)
}
