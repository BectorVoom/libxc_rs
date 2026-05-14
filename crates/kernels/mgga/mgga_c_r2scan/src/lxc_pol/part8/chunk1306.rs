//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1306/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1306<F: Float>(t1299: F, t2941: F, t1256: F, t997: F, t1275: F, t2924: F, t818: F, t9638: F, t1348: F, t9769: F, t2987: F, t6767: F, t10395: F, t18767: F, t18771: F, t18774: F, t23270: F, t23272: F, t4696: F, t4884: F, t8540: F, t8542: F, t8543: F, t9883: F) -> (F, F, F, F, F, F, F) {
    let t31579 = t2941 * t1299;
    let t31608 = t997 * t1256;
    let t31689 = t2924 * t1275;
    let t31764 = t9638 * t818;
    let t31948 = t1348 * t9769;
    let t31953 = t6767 * t2987;
    let t32067 = t9883 + t10395 + t4696 + t18767 + 6.0 * t8540 + 3.0 * t8542 + 6.0 * t8543 + t4884 - t23270 - t18771 + t18774 + t23272;
    (t31579, t31608, t31689, t31764, t31948, t31953, t32067)
}
