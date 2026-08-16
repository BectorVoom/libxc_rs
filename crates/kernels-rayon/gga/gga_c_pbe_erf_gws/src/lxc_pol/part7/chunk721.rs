//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 721/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk721(t1984: f64, t225: f64, t10: f64, t670: f64, t5311: f64, t5314: f64, t5316: f64, t5318: f64, t5324: f64, t5326: f64, t5328: f64, t5330: f64, t5332: f64, t5337: f64, t5339: f64, t5341: f64, t5345: f64, t5348: f64, t5350: f64, t5354: f64) -> (f64, f64, f64) {
    let t5926 = t225 * t1984;
    let t5927 = t10 * t5926;
    let t5929 = 0.32463124087094530131e0_f64 * t670 * t5927;
    let t5930 = -t5311 - t5314 + t5316 - t5318 - t5324 - t5326 - t5328 + t5330 + t5332 + t5337 + t5339 + t5341 + t5345 + t5348 + t5350 + t5354 + t5929;
    (t5926, t5927, t5930)
}
