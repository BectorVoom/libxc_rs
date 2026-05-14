//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1002/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1002<F: Float>(t1276: F, t2391: F, t3366: F, t11050: F, t8358: F, t11885: F, t6654: F, t1010: F, t37040: F, t19155: F, t11880: F, t502: F, t826: F, t11033: F, t8355: F, t12005: F, t1338: F) -> (F, F, F, F, F, F, F, F, F) {
    let t40797 = t1276 * t3366 * t2391;
    let t40798 = 4.0 / 3.0 * t40797;
    let t40804 = t8358 * t11050;
    let t40805 = 4.0 / 3.0 * t40804;
    let t40806 = t6654 * t11885;
    let t40807 = 4.0 / 3.0 * t40806;
    let t40808 = t37040 * t1010;
    let t40815 = t19155 * param_eta;
    let t40821 = t11880 * t502 * t1010 * t826;
    let t40822 = 4.0 * t40821;
    let t40840 = t11033 * t2391;
    let t40841 = 2.0 / 3.0 * t40840;
    let t40844 = t8355 * t3366;
    let t40845 = 2.0 / 3.0 * t40844;
    let t41028 = t1338 * t12005;
    (t40798, t40805, t40807, t40808, t40815, t40822, t40841, t40845, t41028)
}
