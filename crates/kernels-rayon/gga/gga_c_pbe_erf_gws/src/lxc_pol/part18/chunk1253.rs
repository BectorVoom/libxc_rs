//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1253/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1253(t54087: f64, t14099: f64, t863: f64, t885: f64, t1125: f64, t51221: f64, t3179: f64, t51291: f64, t854: f64, t3228: f64, t51465: f64, t3224: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t54088 = 7.0_f64 / 144.0_f64 * t54087;
    let t54090 = t863 * t14099 * t885;
    let t54094 = t1125 * t51221;
    let t54101 = t51291 * t3179;
    let t54102 = t854 * t54101;
    let t54103 = 7.0_f64 / 72.0_f64 * t54102;
    let t54113 = t51465 * t3228;
    let t54114 = 7.0_f64 / 288.0_f64 * t54113;
    let t54117 = t51465 * t3224;
    (t54088, t54090, t54094, t54101, t54103, t54114, t54117)
}
