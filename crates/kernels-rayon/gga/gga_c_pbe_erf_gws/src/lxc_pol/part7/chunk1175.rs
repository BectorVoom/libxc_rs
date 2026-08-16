//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1175/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1175(t2200: f64, t863: f64, t864: f64, t2173: f64, t2157: f64, t6439: f64, t2135: f64, t3138: f64, t3139: f64, t18424: f64, t18428: f64, t18432: f64, t18435: f64, t18439: f64, t18441: f64, t18445: f64, t18448: f64, t18452: f64, t18456: f64, t18460: f64, t18462: f64, t19472: f64) -> (f64, f64, f64) {
    let t20962 = t863 * t864 * t2200;
    let t20963 = t20962 * t2173;
    let t20964 = 35.0_f64 / 18.0_f64 * t20963;
    let t20965 = t2157 * t6439;
    let t20969 = t3138 * t3139 * t2135 * t20965 / 12.0_f64;
    let t20974 = -t19472 + t18424 - t18428 + t18432 - t18435 + t18439 - t18441 - t18445 - t18448 - t18452 + t18456 - t18460 - t18462;
    (t20964, t20969, t20974)
}
