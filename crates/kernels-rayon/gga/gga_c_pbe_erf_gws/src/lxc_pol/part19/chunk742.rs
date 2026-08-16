//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 742/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk742(t2242: f64, t894: f64, t1327: f64, t409: f64, t1285: f64, t1291: f64, t1293: f64, t403: f64, t1274: f64, t405: f64, t1289: f64, t27: f64) -> (f64, f64, f64, f64, f64) {
    let t4487 = t2242 * t894;
    let t4498 = t409 * t1327;
    let t4502 = t1291 * t1285 * t1293 * t403;
    let t4503 = 0.48245472966453314466e2_f64 * t4502;
    let t4505 = t1274 * t405 * t1285;
    let t4506 = 6.0_f64 * t4505;
    let t4508 = 1.0_f64 / t1289 / t27;
    (t4487, t4498, t4503, t4506, t4508)
}
