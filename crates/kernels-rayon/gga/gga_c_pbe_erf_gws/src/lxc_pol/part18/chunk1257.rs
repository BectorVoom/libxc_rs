//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1257/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1257(t3237: f64, t51371: f64, t3242: f64, t3232: f64, t14079: f64, t3283: f64, t1154: f64, t51387: f64, t14046: f64, t3184: f64, t3148: f64, t14023: f64, t14548: f64, t863: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t54283 = t51371 * t3237;
    let t54284 = 7.0_f64 / 576.0_f64 * t54283;
    let t54285 = t51371 * t3242;
    let t54286 = 7.0_f64 / 144.0_f64 * t54285;
    let t54289 = t51371 * t3232;
    let t54290 = 7.0_f64 / 144.0_f64 * t54289;
    let t54301 = t14079 * t3283;
    let t54302 = 7.0_f64 / 576.0_f64 * t54301;
    let t54305 = t51387 * t1154;
    let t54319 = t14046 * t3184;
    let t54320 = 7.0_f64 / 72.0_f64 * t54319;
    let t54322 = t14046 * t3148;
    let t54323 = 7.0_f64 / 72.0_f64 * t54322;
    let t54329 = t863 * t14023 * t14548;
    (t54284, t54286, t54290, t54302, t54305, t54320, t54323, t54329)
}
