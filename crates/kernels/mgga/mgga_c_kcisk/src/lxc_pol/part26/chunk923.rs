//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 923/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk923<F: Float>(t1375: F, t25441: F, t25450: F, t1383: F, t14056: F, t14059: F, t14062: F, t14085: F, t158: F, t165: F, t20660: F, t20670: F, t20676: F, t20679: F, t20687: F, t20718: F, t20719: F, t20736: F, t20739: F, t20752: F, t25342: F, t25413: F, t5816: F, t5827: F) -> (F,) {
    let t25508 = t1375 * t25441;
    let t25511 = t1375 * t25450;
    let t25514 = t1383 * t25441;
    let t25517 = t1383 * t25450;
    let t25528 = -0.21078e-1 * t158 * t25508 + 0.28104e-1 * t5827 * t25511 + 0.4755e-2 * t165 * t25514 - 0.634e-2 * t5816 * t25517 + 0.31368166666666666667e-4 * t20660 + t14056 + t14059 - t14062 + t20670 - t20676 + t20687 - t20718 - 0.31226666666666666667e-2 * t20719 - 0.62154466893555682512e-3 * t14085 * t25342 + 0.62154466893555682512e-3 * t20679 * t25413 + 0.52833333333333333332e-2 * t20736 + t20739 + 0.70444444444444444443e-2 * t20752;
    (t25528,)
}
