//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1281/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1281(t14046: f64, t14522: f64, t3261: f64, t51214: f64, t51306: f64, t9506: f64, t4026: f64, t863: f64, t885: f64, t338: f64, t8828: f64, t14011: f64, t9581: f64) -> (f64, f64, f64, f64, f64) {
    let t54236 = t14046 * t14522;
    let t54238 = t51214 * t3261;
    let t54241 = t51306 * t9506;
    let t54244 = t863 * t4026 * t885;
    let t54246 = t54244 * t338 * t8828;
    let t54248 = t14011 * t9581;
    (t54236, t54238, t54241, t54246, t54248)
}
