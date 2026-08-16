//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1228/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1228(t21243: f64, t21247: f64, t21255: f64, t21267: f64, t21274: f64, t21280: f64, t21286: f64, t21295: f64, t21302: f64, t21306: f64, t21310: f64, t21318: f64, t21326: f64, t21332: f64, t21336: f64, t21338: f64, t21341: f64, t21348: f64, t21355: f64, t21359: f64, t21378: f64, t21382: f64) -> (f64, f64) {
    let t21705 = -t21243 + t21247 + t21255 - t21267 - t21274 + t21280 - t21286 + t21295 + t21302 - t21306 - t21310;
    let t21708 = t21318 + t21326 + t21332 + t21336 + t21338 - t21341 - t21348 - t21355 + t21359 - t21378 + t21382;
    (t21705, t21708)
}
