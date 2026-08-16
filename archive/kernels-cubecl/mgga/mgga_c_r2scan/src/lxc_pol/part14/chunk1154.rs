//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1154/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1154<F: Float>(t10729: F, t40075: F, t25172: F, t3332: F, t6165: F, t25177: F, t7614: F, t11659: F, t6395: F, t10868: F, t7615: F, t11714: F, t6493: F) -> (F, F, F, F, F, F) {
    let t40076 = t40075 * t10729;
    let t40081 = t6165 * t3332 * t25172;
    let t40084 = t7614 * t3332 * t25177;
    let t40086 = t6395 * t11659;
    let t40090 = t7614 * t10868 * t7615;
    let t40092 = t6493 * t11714;
    (t40076, t40081, t40084, t40086, t40090, t40092)
}
