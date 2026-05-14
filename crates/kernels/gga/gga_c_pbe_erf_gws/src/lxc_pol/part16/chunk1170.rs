//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1170/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1170<F: Float>(t14185: F, t2074: F, t2376: F, t2408: F, t2409: F, t2410: F, t27112: F, t3066: F, t335: F, t338: F, t353: F, t4216: F, t4227: F, t53889: F, t53892: F, t53894: F, t53904: F, t53906: F, t54598: F, t54599: F, t55151: F, t55420: F, t55421: F, t55434: F, t55447: F, t55461: F, t55474: F, t55489: F, t55502: F, t55516: F, t55530: F, t55546: F, t55559: F, t55573: F, t55586: F, t55600: F, t55613: F, t55627: F, t55640: F, t55660: F, t55672: F, t8764: F, t898: F, t9283: F) -> (F,) {
    let t55673 = t54598 * t54599 * t4216 * t2410 / 4.0 + t53889 / 48.0 - t53892 / 24.0 - t53894 / 48.0 + t55420 - t55421 - t335 * t338 * t353 * t898 * (t55434 + t55447 + t55461 + t55474 + t55489 + t55502 + t55516 + t55530 + t55546 + t55559 + t55573 + t55586 + t55600 + t55613 + t55627 + t55640) / 96.0 + t3066 * t2409 * t27112 * t4216 / 48.0 + t2408 * t2409 * t2376 * t4227 * t2074 / 48.0 - t55660 + t53904 / 48.0 - t2408 * t9283 * t14185 * t8764 / 24.0 - t2408 * t9283 * t55151 * t2410 / 12.0 + t53906 / 48.0 + t55672;
    (t55673,)
}
