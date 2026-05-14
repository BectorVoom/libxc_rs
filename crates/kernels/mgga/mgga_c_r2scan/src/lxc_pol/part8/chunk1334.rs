//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1334/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1334<F: Float>(t10130: F, t19827: F, t24016: F, t24018: F, t25243: F, t2582: F, t27934: F, t27938: F, t27941: F, t27953: F, t27957: F, t27959: F, t32424: F, t32533: F, t32626: F, t495: F, t5109: F, t6139: F, t6293: F, t7321: F, t8761: F) -> (F,) {
    let t32663 = 0.15602799132097683414e1 * t19827 * t5109 * t10130 * t495 - 0.98781737744032673978e0 * t6293 * t7321 * t32424 - 0.17465477326173296717e-1 * t27934 - 0.17465477326173296717e-1 * t27938 - 0.34930954652346593433e-1 * t27941 + 0.24393601348456957547e-3 * t24016 + 0.4337182319755647052e-4 * t24018 - 0.38087975358139160776e-1 * t27953 - 0.19043987679069580389e-1 * t27957 - 0.7801399566048841707e0 * t6139 * t5109 * t32533 - 0.13002332610081402845e0 * t2582 * t5109 * t32626 - 0.7801399566048841707e0 * t25243 * t8761 + 0.1047928639570397803e0 * t27959;
    (t32663,)
}
