//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 749/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk749<F: Float>(t140: F, t178: F, t9331: F, t9302: F, t9308: F, t9312: F, t9316: F, t9321: F, t9323: F, t9326: F, t9329: F, t218: F, t1009: F, t2685: F, t1053: F, t2692: F, t3181: F) -> (F, F, F, F, F, F) {
    let t9333 = t140 * t178 * t9331;
    let t9335 = -0.10416666666666666667e-1 * t9302 + 0.40208333333333333335e-2 * t9308 - 0.10416666666666666667e-1 * t9312 + 0.24305555555555555556e-1 * t9316 + 0.10416666666666666667e-1 * t9321 + 0.10416666666666666667e-1 * t9323 - 0.13265555555555555555e-1 * t9326 + 0.99491666666666666664e-2 * t9329 - 0.99491666666666666664e-2 * t9333;
    let t9336 = t9335 * t218;
    let t9337 = t2685 * t1009;
    let t9338 = t9337 * t1053;
    let t9339 = t3181 * t2692;
    (t9333, t9335, t9336, t9337, t9338, t9339)
}
