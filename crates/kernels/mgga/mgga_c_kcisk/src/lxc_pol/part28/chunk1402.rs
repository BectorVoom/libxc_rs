//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1402/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1402<F: Float>(t1873: F, t24466: F, t33091: F, t35325: F, t117419: F, t7440: F, t15851: F, t2587: F, t11237: F, t9051: F, t34345: F, t7304: F, t117362: F, t24117: F, t17056: F, t33120: F, t7431: F) -> (F, F, F, F, F, F, F, F) {
    let t122221 = t1873 * t24466;
    let t122223 = t33091 * t35325;
    let t122225 = t117419 * t7440;
    let t122227 = t15851 * t2587;
    let t122229 = t11237 * t9051;
    let t122231 = t34345 * t7304;
    let t122233 = t117362 * t24117;
    let t122236 = t17056 * t33120 * t7431;
    (t122221, t122223, t122225, t122227, t122229, t122231, t122233, t122236)
}
