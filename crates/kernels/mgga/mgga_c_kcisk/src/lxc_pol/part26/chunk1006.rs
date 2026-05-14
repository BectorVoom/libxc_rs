//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1006/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1006<F: Float>(t19055: F, t6230: F, t5886: F, t5968: F, t1411: F, t5606: F, t5981: F, t1440: F, t3797: F, t7710: F, t3796: F, t3482: F, t5874: F, t6217: F, t1286: F, t7906: F) -> (F, F, F, F, F, F, F) {
    let t26964 = t19055 * t6230;
    let t26966 = t5886 * t5968;
    let t26967 = t1411 * t26966;
    let t26969 = t5606 * t5981;
    let t26970 = t1411 * t26969;
    let t26974 = t3797 * t7710 * t1440;
    let t26975 = t3796 * t26974;
    let t26976 = t3482 * t26975;
    let t26980 = t5874 * t6217;
    let t26987 = t7906 * t1286;
    (t26964, t26967, t26970, t26974, t26976, t26980, t26987)
}
