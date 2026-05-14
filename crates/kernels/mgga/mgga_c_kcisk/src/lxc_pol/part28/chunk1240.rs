//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1240/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1240<F: Float>(t35130: F, t35173: F, t35210: F, t35260: F, t752: F, t2594: F, t34288: F, t33071: F, t8968: F, t9094: F, t9696: F, t24081: F, t2799: F, t17775: F, t9967: F, t7293: F, t9988: F) -> (F, F, F, F, F, F, F, F) {
    let t35262 = t35130 + t35173 + t35210 + t35260;
    let t35263 = t35262 * t752;
    let t35265 = 2.0 * t34288 * t2594;
    let t35267 = 2.0 * t33071 * t8968;
    let t35268 = t9696 * t9094;
    let t35269 = t24081 * t2799;
    let t35271 = 4.0 * t17775 * t9967;
    let t35273 = 2.0 * t7293 * t9988;
    (t35262, t35263, t35265, t35267, t35268, t35269, t35271, t35273)
}
