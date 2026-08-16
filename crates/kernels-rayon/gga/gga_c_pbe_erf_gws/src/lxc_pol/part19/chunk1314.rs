//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1314/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1314(t14093: f64, t57030: f64, t2080: f64, t3803: f64, t852: f64, t14092: f64, t6341: f64, t14064: f64, t3805: f64, t1184: f64, t12000: f64, t11535: f64, t14031: f64) -> (f64, f64, f64, f64, f64) {
    let t57031 = t57030 * t14093;
    let t57034 = t2080 * t3803 * t852;
    let t57036 = t57034 * t14092 * t6341;
    let t57038 = t3805 * t14064;
    let t57040 = t1184 * t12000;
    let t57042 = t14031 * t11535;
    (t57031, t57036, t57038, t57040, t57042)
}
