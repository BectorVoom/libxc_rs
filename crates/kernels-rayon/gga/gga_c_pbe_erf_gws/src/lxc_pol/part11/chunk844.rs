//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 844/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk844(t13334: f64, t343: f64, t858: f64, t867: f64, t866: f64, t274: f64, t3772: f64, t2255: f64, t3258: f64, t11495: f64, t11497: f64, t13284: f64, t13287: f64, t13295: f64, t13296: f64, t13302: f64, t13306: f64, t13308: f64, t13309: f64, t13313: f64, t13314: f64, t2277: f64, t2312: f64, t6579: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13335 = t13334 * t343;
    let t13337 = t867 * t858 * t13335;
    let t13339 = t866 * t13337 / 96.0_f64;
    let t13340 = t274 * t3772;
    let t13342 = t2255 * t3258 * t13340;
    let t13345 = t13284 + t2277 * t13287 / 256.0_f64 + t13295 + 5.0_f64 / 128.0_f64 * t6579 * t13296 + t13302 - t13306 - t13308 - t2312 * t13309 / 128.0_f64 - t13313 + t13314 + 7.0_f64 / 768.0_f64 * t11495 + 7.0_f64 / 768.0_f64 * t11497 - t13339 - t2277 * t13342 / 1536.0_f64;
    (t13335, t13337, t13339, t13340, t13342, t13345)
}
