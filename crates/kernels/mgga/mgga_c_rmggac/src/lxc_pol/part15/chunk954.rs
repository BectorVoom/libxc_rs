//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 954/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk954<F: Float>(t38530: F, t8432: F, t8437: F, t26287: F, t46441: F, t26283: F, t46444: F, t26291: F, t46397: F, t10112: F, t2157: F, t2868: F, t8997: F, t1971: F, t236: F, t6099: F, t8517: F) -> (F, F, F, F, F, F, F, F) {
    let t47966 = t38530 * t8432;
    let t47968 = t38530 * t8437;
    let t47970 = t26287 * t46441;
    let t47972 = t26283 * t46444;
    let t47974 = t26291 * t46397;
    let t47976 = t10112 * t2157;
    let t47980 = t2868 * t8997;
    let t47984 = t8517 * t1971 * t236 * t6099;
    (t47966, t47968, t47970, t47972, t47974, t47976, t47980, t47984)
}
