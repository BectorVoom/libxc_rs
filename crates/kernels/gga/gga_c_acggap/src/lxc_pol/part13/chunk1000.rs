//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1000/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1000<F: Float>(t1980: F, t35500: F, t7476: F, t31262: F, t31277: F, t31279: F, t1089: F, t15897: F, t2288: F, t598: F, t1988: F, t8486: F, t1967: F, t8838: F, t4352: F, t535: F, t7656: F) -> (F, F, F, F, F, F, F, F) {
    let t35502 = t1980 * t7476 * t35500;
    let t35503 = 0.7145669686344956162e-3 * t35502;
    let t35506 = 0.26147916666666666666e0 * t31262;
    let t35507 = 0.3973125e0 * t31277;
    let t35508 = 0.264875e0 * t31279;
    let t35511 = t598 * t1089 * t15897 * t2288;
    let t35513 = t1988 * t8486;
    let t35514 = 0.94344276868812456204e-3 * t35513;
    let t35515 = t1967 * t8838;
    let t35519 = t598 * t4352 * t535 * t7656;
    (t35503, t35506, t35507, t35508, t35511, t35514, t35515, t35519)
}
