//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 874/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk874<F: Float>(t3479: F, t636: F, t3493: F, t3397: F, t577: F, t184: F, t199: F, t7778: F, t3399: F, t612: F, t1004: F, t562: F, t997: F, t7171: F, t5465: F, t5418: F, t5423: F, t5429: F, t5430: F, t5433: F, t5436: F, t5437: F, t5443: F, t7775: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10887 = t3479 * t636;
    let t10888 = 4.0 / 45.0 * t10887;
    let t10889 = t3493 * t636;
    let t10890 = 8.0 / 45.0 * t10889;
    let t10891 = t3397 * t577;
    let t10892 = t10891 * t184;
    let t10894 = 4.0 / 15.0 * t10892 * t199;
    let t10895 = 16.0 / 45.0 * t7778;
    let t10897 = 4.0 / 15.0 * t3399 * t612;
    let t10898 = t562 * t1004;
    let t10899 = t10898 * t184;
    let t10901 = 8.0 / 15.0 * t10899 * t997;
    let t10903 = 8.0 / 15.0 * t7171 * t997;
    let t10904 = 8.0 / 405.0 * t5465;
    let t10905 = 0.12155555555555555555e0 * t5418 + t5423 + t5429 + 4.0 / 9.0 * t5430 + t5433 + t5436 - 2.0 / 27.0 * t5437 - t5443 + t10888 + t10890 + t10894 + t7775 - t10895 - t10897 + t10901 + t10903 - t10904;
    (t10888, t10890, t10894, t10895, t10897, t10901, t10903, t10904, t10905)
}
