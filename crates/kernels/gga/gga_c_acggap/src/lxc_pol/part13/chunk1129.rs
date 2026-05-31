//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1129/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1129<F: Float>(t30268: F, t8783: F, t31254: F, t1479: F, t429: F, t1980: F, t7476: F, t31262: F, t31277: F, t31279: F, t1089: F, t15897: F, t2288: F, t598: F) -> (F, F, F, F, F, F, F, F) {
    let t35496 = t30268 * t8783;
    let t35497 = F::cast_from(0.94344276868812456204e-2_f64) * t35496;
    let t35499 = F::cast_from(0.85748036236139473944e-3_f64) * t31254;
    let t35500 = t429 * t1479;
    let t35502 = t1980 * t7476 * t35500;
    let t35503 = F::cast_from(0.7145669686344956162e-3_f64) * t35502;
    let t35506 = F::cast_from(0.26147916666666666666e0_f64) * t31262;
    let t35507 = F::cast_from(0.3973125e0_f64) * t31277;
    let t35508 = F::cast_from(0.264875e0_f64) * t31279;
    let t35511 = t598 * t1089 * t15897 * t2288;
    (t35497, t35499, t35500, t35503, t35506, t35507, t35508, t35511)
}
