//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 868/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk868<F: Float>(t1065: F, t792: F, t11002: F, t1102: F, t3314: F, t3457: F, t2304: F, t875: F, t3434: F, t3439: F, t1266: F, t321: F, t502: F, t818: F, t826: F, t1275: F, t263: F) -> (F, F, F, F, F, F, F, F) {
    let t11003 = t1065 * t792;
    let t11004 = t11002 * t11003;
    let t11008 = t1102 * t3314 * t3457;
    let t11015 = t2304 * t875;
    let t11017 = t3434 * t11015 * t3439;
    let t11018 = 0.1951603679568577289e-3 * t11017;
    let t11031 = t1266 * t321;
    let t11032 = 11.0 / 9.0 * t11031;
    let t11033 = t502 * t818;
    let t11034 = t11033 * t826;
    let t11036 = t263 * t1275;
    (t11004, t11008, t11015, t11018, t11032, t11033, t11034, t11036)
}
