//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 849/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk849<F: Float>(t28663: F, t5497: F, t1775: F, t5486: F, t5006: F, t2642: F, t7715: F, t12271: F, t2364: F, t25045: F, t7718: F, t5491: F, t12234: F, t12248: F, t2013: F, t24976: F, t24978: F, t24980: F, t25007: F, t25024: F, t25027: F, t7581: F, t9214: F) -> (F,) {
    let t29811 = t5497 * t28663;
    let t29812 = t1775 * t29811;
    let t29815 = t5486 * t28663;
    let t29816 = t5006 * t29815;
    let t29819 = t7715 * t2642;
    let t29820 = t12271 * t29819;
    let t29821 = t5006 * t29820;
    let t29825 = t25045 * t2364;
    let t29826 = t1775 * t29825;
    let t29831 = t7718 * t2642;
    let t29832 = t5491 * t29831;
    let t29833 = t1775 * t29832;
    let t29836 = t12234 * t29819;
    let t29837 = t1775 * t29836;
    let t29842 = 0.2698618307426597582e-1 * t24976 - 0.14392630972941853771e0 * t24978 + 0.26386490117060065246e0 * t24980 - 0.53972366148531951639e-1 * t2013 * t29812 + 0.35981577432354634428e-1 * t2013 * t29816 - 0.35981577432354634427e-1 * t2013 * t29821 + t12248 - 0.17990788716177317214e-1 * t25007 + 0.53972366148531951639e-1 * t2013 * t29826 - 0.53972366148531951639e-1 * t7581 * t9214 - 0.2698618307426597582e-1 * t2013 * t29833 + 0.53972366148531951639e-1 * t2013 * t29837 - 0.53972366148531951639e-1 * t25024 - 0.2698618307426597582e-1 * t25027;
    (t29842,)
}
