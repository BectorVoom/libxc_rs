//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1099/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1099<F: Float>(t40797: F, t1070: F, t1276: F, t8395: F, t11047: F, t23498: F, t11050: F, t8358: F, t11885: F, t6654: F, t1010: F, t37040: F, t11882: F, t19146: F, t37043: F, t37048: F, t37055: F, t37069: F, t40779: F, t40782: F, t40786: F, t40788: F, t40790: F, t40792: F, t40794: F) -> (F,) {
    let t40798 = 4.0 / 3.0 * t40797;
    let t40800 = t1276 * t1070 * t8395;
    let t40802 = t23498 * t11047;
    let t40804 = t8358 * t11050;
    let t40805 = 4.0 / 3.0 * t40804;
    let t40806 = t6654 * t11885;
    let t40807 = 4.0 / 3.0 * t40806;
    let t40808 = t37040 * t1010;
    let t40812 = t19146 * param_eta * t11882;
    let t40814 = -11.0 / 9.0 * t40779 + t40782 - 4.0 / 3.0 * t37048 + 2.0 * t37055 - 2.0 / 3.0 * t37069 + t40786 + 22.0 / 9.0 * t40788 + t40790 / 4.0 + t40792 / 4.0 + t40794 / 2.0 - t40798 + t40800 / 4.0 - 3.0 / 4.0 * t40802 - t40805 - t40807 + 11.0 / 9.0 * t40808 - t37043 / 3.0 - 3.0 / 2.0 * t40812;
    (t40814,)
}
