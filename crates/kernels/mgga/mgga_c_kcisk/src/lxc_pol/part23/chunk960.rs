//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 960/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk960<F: Float>(t19512: F, t5744: F, t1186: F, t19136: F, t26: F, t19132: F, t3651: F, t5684: F, t1175: F, t12929: F, t19134: F, t19138: F, t19142: F, t19212: F, t19214: F, t19510: F) -> (F, F, F, F, F) {
    let t19513 = t5744 * t19512;
    let t19515 = t1186 * t19136;
    let t19516 = t26 * t19515;
    let t19518 = t1186 * t19132;
    let t19519 = t5744 * t19518;
    let t19523 = t3651 * t5684;
    let t19524 = t19523 * t1175;
    let t19526 = 0.23917333333333333334e1 * t19134 + 0.59793333333333333334e0 * t19138 - 0.59793333333333333334e0 * t19142 - 0.26574814814814814816e0 * t12929 - 0.16431333333333333333e0 * t19510 - 0.10954222222222222222e0 * t19513 + 0.16431333333333333333e0 * t19516 + 0.65725333333333333332e0 * t19519 - 0.1898925e1 * t19212 - 0.9494625e0 * t19214 + 0.3071625e0 * t19524;
    (t19513, t19516, t19519, t19524, t19526)
}
