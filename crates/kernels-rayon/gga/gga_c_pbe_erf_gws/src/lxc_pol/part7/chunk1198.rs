//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1198/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1198(t21033: f64, t858: f64, t866: f64, t867: f64, t19553: f64, t21269: f64, t21274: f64, t21280: f64, t21286: f64, t21287: f64, t21295: f64, t21302: f64, t21306: f64, t2343: f64, t2345: f64, t6220: f64, t6308: f64, t6555: f64, t904: f64, t916: f64, t929: f64, t933: f64) -> (f64, f64) {
    let t21310 = t866 * t867 * t858 * t21033 / 96.0_f64;
    let t21311 = 595.0_f64 / 576.0_f64 * t21269 - t21274 - t929 * t933 * t904 * t19553 / 768.0_f64 + t21280 + t2343 * t2345 * t6308 * t6220 / 64.0_f64 - t21286 - 3.0_f64 / 64.0_f64 * t6555 * t916 * t904 * t21287 + t21295 + t21302 - t21306 - t21310;
    (t21310, t21311)
}
