//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 427/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk427<F: Float>(t1136: F, t1139: F, t1138: F, t288: F, t285: F, t1147: F, t3234: F, t3237: F, t3239: F, t3243: F, t3246: F, t3249: F, t3251: F, t3254: F, t3256: F, t3258: F, t3261: F, t3264: F, t3267: F, t3272: F) -> (F, F, F, F, F) {
    let t3437 = t1136 * t1139;
    let t3441 = 1.0 / t1138 / t288;
    let t3442 = t285 * t3441;
    let t3443 = t1147 * t1147;
    let t3460 = 0.1875e0 * t3234 - 0.375e0 * t3237 - 0.75e0 * t3239 + 0.375e0 * t3243 + 0.75e0 * t3246 - 0.1875e0 * t3249 + 0.1125e1 * t3251 - 0.4046875e-1 * t3254 + 0.809375e-1 * t3256 + 0.32375e0 * t3258 - 0.809375e-1 * t3261 - 0.32375e0 * t3264 + 0.4046875e-1 * t3267 - 0.809375e0 * t3272;
    (t3437, t3441, t3442, t3443, t3460)
}
