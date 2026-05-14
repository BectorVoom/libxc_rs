//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1048/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1048<F: Float>(t40076: F, t25172: F, t3332: F, t6165: F, t25177: F, t7614: F, t11659: F, t6395: F, t38055: F, t38056: F, t38062: F, t40059: F, t40064: F, t40068: F, t40070: F, t40073: F) -> (F,) {
    let t40077 = 0.47609969197673950972e-2 * t40076;
    let t40081 = t6165 * t3332 * t25172;
    let t40084 = t7614 * t3332 * t25177;
    let t40086 = t6395 * t11659;
    let t40087 = 0.46574606203128791246e-1 * t40086;
    let t40088 = 0.86682217400542685632e-1 * t40059 + 0.87327386630866483584e-2 * t40064 + 0.26198215989259945076e-1 * t40068 - 0.59512461497092438715e-1 * t40070 + 0.13002332610081402845e0 * t40073 + t40077 - t38055 - 0.11557628986739024751e0 * t38056 + 0.46574606203128791246e-1 * t38062 - 0.13099107994629972538e-1 * t40081 - 0.5239643197851989015e-1 * t40084 - t40087;
    (t40088,)
}
