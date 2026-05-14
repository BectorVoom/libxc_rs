//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 895/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk895<F: Float>(t10114: F, t1057: F, t2466: F, t1065: F, t2471: F, t323: F, t325: F, t8291: F, t41: F, t4879: F, t335: F, t333: F, t3110: F, t317: F, t319: F, t1027: F, t3075: F) -> (F, F, F, F, F, F, F, F) {
    let t10115 = 0.62154466893555682512e-3 * t10114;
    let t10131 = t2466 * t1057;
    let t10133 = t2471 * t1065;
    let t10137 = 0.77488888888888888888e-2 * t323 * t8291 * t325;
    let t10138 = t4879 * t41;
    let t10139 = t10138 * t335;
    let t10141 = 0.72818958333333333333e-4 * t333 * t10139;
    let t10144 = 0.27323333333333333333e-1 * t317 * t3110 * t319;
    let t10150 = t1027 * t3075;
    (t10115, t10131, t10133, t10137, t10138, t10141, t10144, t10150)
}
