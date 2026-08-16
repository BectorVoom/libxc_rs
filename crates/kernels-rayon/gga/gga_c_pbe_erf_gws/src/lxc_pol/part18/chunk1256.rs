//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1256/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1256(t4026: f64, t828: f64, t3287: f64, t51255: f64, t3142: f64, t51382: f64, t1125: f64, t51292: f64, t14024: f64, t3120: f64, t21296: f64, t367: f64, t899: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t54253 = t4026 * t828;
    let t54257 = t51255 * t3287;
    let t54258 = 7.0_f64 / 144.0_f64 * t54257;
    let t54259 = t51382 * t3142;
    let t54260 = 7.0_f64 / 72.0_f64 * t54259;
    let t54267 = t1125 * t51292;
    let t54268 = 7.0_f64 / 72.0_f64 * t54267;
    let t54271 = t3120 * t14024;
    let t54272 = 7.0_f64 / 144.0_f64 * t54271;
    let t54279 = t899 * t21296 * t367;
    (t54253, t54258, t54260, t54268, t54272, t54279)
}
