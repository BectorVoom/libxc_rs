//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 844/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk844<F: Float>(t13334: F, t343: F, t858: F, t867: F, t866: F, t274: F, t3772: F, t2255: F, t3258: F, t11495: F, t11497: F, t13284: F, t13287: F, t13295: F, t13296: F, t13302: F, t13306: F, t13308: F, t13309: F, t13313: F, t13314: F, t2277: F, t2312: F, t6579: F) -> (F, F, F, F, F, F) {
    let t13335 = t13334 * t343;
    let t13337 = t867 * t858 * t13335;
    let t13339 = t866 * t13337 / F::new(96.0);
    let t13340 = t274 * t3772;
    let t13342 = t2255 * t3258 * t13340;
    let t13345 = t13284 + t2277 * t13287 / F::new(256.0) + t13295 + F::new(5.0) / F::new(128.0) * t6579 * t13296 + t13302 - t13306 - t13308 - t2312 * t13309 / F::new(128.0) - t13313 + t13314 + F::new(7.0) / F::new(768.0) * t11495 + F::new(7.0) / F::new(768.0) * t11497 - t13339 - t2277 * t13342 / F::new(1536.0);
    (t13335, t13337, t13339, t13340, t13342, t13345)
}
