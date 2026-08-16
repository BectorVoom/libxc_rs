//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1193/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1193(t6161: f64, t745: f64, t2100: f64, t1452: f64, t274: f64, t6084: f64, t2197: f64, t6228: f64, t2164: f64, t6520: f64, t2084: f64, t21183: f64, t21187: f64, t21191: f64, t21196: f64, t2255: f64, t2277: f64, t2278: f64, t3257: f64, t3259: f64, t6350: f64, t6573: f64, t6664: f64) -> (f64, f64, f64) {
    let t21201 = t745 * t6161;
    let t21206 = t745 * t2100;
    let t21211 = t1452 * t274;
    let t21216 = t274 * t6084;
    let t21221 = t6228 * t2197;
    let t21222 = 35.0_f64 / 72.0_f64 * t21221;
    let t21223 = t2164 * t6520;
    let t21224 = 7.0_f64 / 72.0_f64 * t21223;
    let t21225 = t21183 + t21187 - t21191 - t2277 * t2255 * t6350 * t6573 / 512.0_f64 + t2277 * t3257 * t21196 * t3259 / 192.0_f64 + t2277 * t2255 * t6664 * t21201 / 256.0_f64 - t2277 * t2255 * t2278 * t21206 / 512.0_f64 + t2277 * t3257 * t2084 * t21211 / 192.0_f64 - t2277 * t2255 * t2278 * t21216 / 1536.0_f64 - t21222 + t21224;
    (t21222, t21224, t21225)
}
