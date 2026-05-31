//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1193/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1193<F: Float>(t6161: F, t745: F, t2100: F, t1452: F, t274: F, t6084: F, t2197: F, t6228: F, t2164: F, t6520: F, t2084: F, t21183: F, t21187: F, t21191: F, t21196: F, t2255: F, t2277: F, t2278: F, t3257: F, t3259: F, t6350: F, t6573: F, t6664: F) -> (F, F, F) {
    let t21201 = t745 * t6161;
    let t21206 = t745 * t2100;
    let t21211 = t1452 * t274;
    let t21216 = t274 * t6084;
    let t21221 = t6228 * t2197;
    let t21222 = F::cast_from(35.0_f64) / F::cast_from(72.0_f64) * t21221;
    let t21223 = t2164 * t6520;
    let t21224 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t21223;
    let t21225 = t21183 + t21187 - t21191 - t2277 * t2255 * t6350 * t6573 / F::cast_from(512.0_f64) + t2277 * t3257 * t21196 * t3259 / F::cast_from(192.0_f64) + t2277 * t2255 * t6664 * t21201 / F::cast_from(256.0_f64) - t2277 * t2255 * t2278 * t21206 / F::cast_from(512.0_f64) + t2277 * t3257 * t2084 * t21211 / F::cast_from(192.0_f64) - t2277 * t2255 * t2278 * t21216 / F::cast_from(1536.0_f64) - t21222 + t21224;
    (t21222, t21224, t21225)
}
