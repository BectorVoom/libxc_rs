//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1131/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1131<F: Float>(t1727: F, t3303: F, t3275: F, t10415: F, t1670: F, t127: F, t2840: F, t368: F, t1109: F, t2844: F, t14303: F, t1114: F) -> (F, F, F, F, F) {
    let t14312 = t3303 * t1727;
    let t14313 = t14312 * t3275;
    let t14316 = t10415 * t1670;
    let t14317 = t14316 * t3275;
    let t14321 = t127 * t368 * t2840;
    let t14322 = t1109 * t2844;
    let t14323 = t14322 * t14303;
    let t14326 = t1114 * t2844;
    (t14313, t14317, t14321, t14323, t14326)
}
