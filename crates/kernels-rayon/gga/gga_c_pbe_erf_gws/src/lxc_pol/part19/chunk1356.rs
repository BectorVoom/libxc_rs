//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1356/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1356(t1105: f64, t14311: f64, t15081: f64, t2376: f64, t2408: f64, t2409: f64, t3921: f64, t52989: f64, t54911: f64, t54915: f64, t54923: f64, t54927: f64, t56166: f64, t56168: f64, t56170: f64, t56174: f64, t56176: f64, t56181: f64, t56190: f64, t56194: f64) -> f64 {
    let t58011 = t52989 - t56166 / 768.0_f64 - t56168 / 12.0_f64 + t54911 + t56170 / 4.0_f64 + t54915 - t56174 / 768.0_f64 + t56176 / 12.0_f64 - t3921 * t14311 / 96.0_f64 + t56181 / 384.0_f64 + t2408 * t2409 * t2376 * t15081 * t1105 / 24.0_f64 + t54923 - t56190 / 24.0_f64 - t56194 / 192.0_f64 - t54927;
    t58011
}
