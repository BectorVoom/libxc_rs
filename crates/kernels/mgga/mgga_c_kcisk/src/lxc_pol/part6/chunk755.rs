//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 755/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk755<F: Float>(t2364: F, t9029: F, t5184: F, t5182: F, t2441: F, t5193: F, t7718: F, t5192: F, t6974: F, t8947: F, t1869: F, t6719: F, t8866: F, t1799: F, t6965: F, t8946: F) -> (F, F, F, F, F) {
    let t28275 = t9029 * t2364;
    let t28276 = t5184 * t28275;
    let t28277 = t5182 * t28276;
    let t28280 = t5193 * t7718 * t2441;
    let t28281 = t5192 * t28280;
    let t28282 = t5182 * t28281;
    let t28284 = t6974 * t8947;
    let t28285 = t1869 * t28284;
    let t28287 = t6719 * t8866;
    let t28288 = t1799 * t28287;
    let t28290 = t6965 * t8946;
    (t28277, t28282, t28285, t28288, t28290)
}
