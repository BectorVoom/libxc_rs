//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1346/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1346(t14937: f64, t9270: f64, t14895: f64, t8801: f64, t14185: f64, t2074: f64, t2376: f64, t2408: f64, t2409: f64, t2410: f64, t27112: f64, t3066: f64, t335: f64, t338: f64, t353: f64, t4216: f64, t4227: f64, t53889: f64, t53892: f64, t53894: f64, t53904: f64, t53906: f64, t54598: f64, t54599: f64, t55151: f64, t55420: f64, t55421: f64, t55434: f64, t55447: f64, t55461: f64, t55474: f64, t55489: f64, t55502: f64, t55516: f64, t55530: f64, t55546: f64, t55559: f64, t55573: f64, t55586: f64, t55600: f64, t55613: f64, t55627: f64, t55640: f64, t8764: f64, t898: f64, t9283: f64) -> f64 {
    let t55660 = 7.0_f64 / 72.0_f64 * t9270 * t14937;
    let t55672 = 7.0_f64 / 24.0_f64 * t8801 * t14895;
    let t55673 = t54598 * t54599 * t4216 * t2410 / 4.0_f64 + t53889 / 48.0_f64 - t53892 / 24.0_f64 - t53894 / 48.0_f64 + t55420 - t55421 - t335 * t338 * t353 * t898 * (t55434 + t55447 + t55461 + t55474 + t55489 + t55502 + t55516 + t55530 + t55546 + t55559 + t55573 + t55586 + t55600 + t55613 + t55627 + t55640) / 96.0_f64 + t3066 * t2409 * t27112 * t4216 / 48.0_f64 + t2408 * t2409 * t2376 * t4227 * t2074 / 48.0_f64 - t55660 + t53904 / 48.0_f64 - t2408 * t9283 * t14185 * t8764 / 24.0_f64 - t2408 * t9283 * t55151 * t2410 / 12.0_f64 + t53906 / 48.0_f64 + t55672;
    t55673
}
