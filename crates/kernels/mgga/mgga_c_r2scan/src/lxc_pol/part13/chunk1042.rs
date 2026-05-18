//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1042/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1042<F: Float>(t3270: F, t36987: F, t1065: F, t2259: F, t11002: F, t11060: F, t833: F, t1299: F, t3370: F, t1074: F, t6692: F, t1275: F, t502: F) -> (F, F, F, F, F, F) {
    let t36988 = t3270 * t36987;
    let t36994 = t1065 * t2259;
    let t36995 = t11002 * t36994;
    let t37015 = t11060 * t833;
    let t37020 = t3370 * t1299;
    let t37023 = t1074 * t6692;
    let t37028 = t502 * t1275;
    (t36988, t36995, t37015, t37020, t37023, t37028)
}
