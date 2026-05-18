//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 643/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk643<F: Float>(t3503: F, t590: F, t587: F, t2750: F, t2754: F, t2757: F, t2797: F, t2014: F, t3481: F, t3490: F, t3495: F, t3496: F, t3498: F, t3502: F) -> (F, F, F, F, F, F, F) {
    let t3504 = t590 * t3503;
    let t3506 = F::new(8.0) / F::new(45.0) * t587 * t3504;
    let t3507 = F::new(8.0) / F::new(45.0) * t2750;
    let t3508 = F::new(16.0) / F::new(45.0) * t2754;
    let t3509 = F::new(8.0) / F::new(45.0) * t2757;
    let t3510 = F::new(16.0) / F::new(45.0) * t2797;
    let t3511 = t3481 + t3490 + t3495 + t3496 + t3498 - t3502 - t3506 - t3507 + t3508 - t3509 + t2014 + t3510;
    (t3504, t3506, t3507, t3508, t3509, t3510, t3511)
}
