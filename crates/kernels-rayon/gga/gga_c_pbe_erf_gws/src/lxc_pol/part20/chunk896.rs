//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 896/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk896(t43: f64, t338: f64, t3907: f64, t939: f64, t3896: f64, t892: f64, t3737: f64, t3887: f64, t3342: f64, t4757: f64, t1402: f64, t3346: f64, t1351: f64, t2457: f64, t418: f64, t47: f64, t9788: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64) {
    let t44 = t43 <= zeta_threshold;
    let t9965 = t338 * t3907 * t939;
    let t9969 = t338 * t892 * t3896;
    let t9973 = t338 * t892 * t3737;
    let t9978 = t338 * t892 * t3887;
    let t9981 = t4757 * t3342;
    let t9986 = t1402 * t3346;
    let t9992 = piecewise3(t44, 0.0_f64, -8.0_f64 / 27.0_f64 * t9981 * t418 + 16.0_f64 / 9.0_f64 * t2457 * t1351 + 4.0_f64 / 9.0_f64 * t9986 * t418 + 4.0_f64 / 3.0_f64 * t47 * t9788);
    (t9965, t9969, t9973, t9978, t9992)
}
