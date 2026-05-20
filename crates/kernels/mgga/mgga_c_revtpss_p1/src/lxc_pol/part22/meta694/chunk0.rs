//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2699/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2699<F: Float>(t3930: F, t6846: F, t221: F, t4019: F, t6862: F, t10001: F, t6800: F, t72: F, t757: F, t1317: F, t6801: F, t13599: F, t21901: F, t21905: F, t21933: F, t9278: F, t9308: F, t9316: F, t9320: F, t9325: F, t9329: F, t9333: F, t9374: F, t9389: F, t9391: F) -> (F, F, F, F, F, F, F) {
    let t22179 = t3930 * t6846;
    let t22182 = t4019 * t221 * t6862;
    let t22183 = t10001 * t22182;
    let t22185 = t6800 * t72;
    let t22186 = t22185 * t757;
    let t22187 = F::cast_from(0.18311447306006545054e-3_f64) * t22186;
    let t22188 = t1317 * t6801;
    let t22189 = F::new(4.0) * t22188;
    let t22190 = t21901 - t9278 + t9308 + t9316 + t9320 - t9325 + t9329 + t9333 - t9374 - t21905 - t9389 - t13599 + t21933 - t9391 - t22187 + t22189;
    (t22179, t22182, t22183, t22185, t22187, t22189, t22190)
}
