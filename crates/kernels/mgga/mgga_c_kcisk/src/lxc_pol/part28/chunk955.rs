//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 955/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk955<F: Float>(t22506: F, t7000: F, t11314: F, t11400: F, t1421: F, t16810: F, t16844: F, t22431: F, t22436: F, t22441: F, t22447: F, t22452: F, t22456: F, t22461: F, t22466: F, t22469: F, t22473: F, t22477: F, t22481: F, t22485: F, t22489: F, t22493: F, t22497: F, t22502: F, t5913: F) -> (F,) {
    let t22507 = t7000 * t22506;
    let t22510 = 0.32852148333333333333e-2 * t16844 * t22431 - 0.19711289e-2 * t11400 * t22436 + 0.26281718666666666666e-2 * t11400 * t22441 - 0.87605728888888888887e-3 * t16810 + 0.492782225e-3 * t1421 * t22447 - 0.1478346675e-2 * t1421 * t22452 + 0.59133867e-2 * t1421 * t22456 - 0.65704296666666666667e-3 * t1421 * t22461 + 0.13140859333333333333e-2 * t1421 * t22466 + 0.492782225e-3 * t22469 - 0.14600954814814814815e-3 * t11314 + 0.26281718666666666666e-2 * t5913 * t22473 - 0.19711289e-2 * t1421 * t22477 + 0.13140859333333333333e-2 * t1421 * t22481 + 0.39422577999999999999e-2 * t1421 * t22485 + 0.52563437333333333332e-2 * t5913 * t22489 + 0.98556445e-3 * t1421 * t22493 - 0.65704296666666666667e-3 * t1421 * t22497 - 0.13140859333333333333e-2 * t1421 * t22502 + 0.10950716111111111111e-2 * t1421 * t22507;
    (t22510,)
}
