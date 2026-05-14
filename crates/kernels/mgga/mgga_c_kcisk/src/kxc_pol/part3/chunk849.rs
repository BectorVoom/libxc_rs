//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 849/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk849<F: Float>(t12957: F, t1383: F, t1398: F, t1375: F, t14014: F, t14016: F, t14019: F, t14022: F, t14025: F, t14027: F, t14029: F, t14031: F, t14033: F, t158: F, t165: F, t173: F) -> (F,) {
    let t14036 = t1383 * t12957;
    let t14039 = t1398 * t12957;
    let t14042 = t1375 * t12957;
    let t14045 = 0.26416666666666666666e-2 * t14014 + 0.7925e-3 * t165 * t14016 - 0.17611111111111111111e-3 * t165 * t14019 - 0.7026e-2 * t158 * t14022 + 0.79249999999999999999e-2 * t14025 + 0.10566666666666666666e-1 * t14027 + 0.117630625e-3 * t14029 + 0.70578375e-4 * t14031 + 0.317e-2 * t165 * t14033 + 0.4755e-2 * t165 * t14036 + 0.30247875e-4 * t173 * t14039 - 0.21078e-1 * t158 * t14042;
    (t14045,)
}
