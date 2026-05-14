//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1372/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1372<F: Float>(t32203: F, t33366: F, t5600: F, t33459: F, t3969: F, t1292: F, t1308: F, t6221: F, t109883: F, t19822: F, t3482: F, t109882: F, t470: F, t19721: F, t19725: F, t5633: F) -> (F, F, F, F, F, F) {
    let t114223 = t5600 * t32203 * t33366;
    let t114225 = t33459 * t3969;
    let t114231 = t6221 * t1292 * t1308;
    let t114241 = t3482 * t109883 * t19822;
    let t114243 = t109882 * t470;
    let t114245 = t3482 * t114243 * t19721;
    let t114248 = t5633 * t114243 * t19725;
    (t114223, t114225, t114231, t114241, t114245, t114248)
}
