//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 358/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk358<F: Float>(t3295: F, t969: F, t825: F, t3191: F, t325: F, t3190: F, t813: F, t2685: F, t2684: F, t894: F, t988: F, t2268: F, t3094: F, t3107: F, t3099: F, t3104: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t3296 = t969 * t3295;
    let t3297 = t825 * t3296;
    let t3298 = 0.38342925953920749676e0 * t3297;
    let t3307 = t3191 * t325;
    let t3308 = t3190 * t3307;
    let t3309 = t813 * t3308;
    let t3311 = t2685 * t3295;
    let t3312 = t2684 * t3311;
    let t3313 = 0.38342925953920749676e0 * t3312;
    let t3327 = t894 * t988;
    let t3329 = 0.28455006635676149599e-1 * t2268 * t3327;
    let t3330 = 3.0 / 128.0 * t3094;
    let t3333 = t3107 / 128.0;
    let t3334 = t3330 - 9.0 / 4096.0 * t3099 + 3.0 / 4096.0 * t3104 - t3333;
    (t3296, t3298, t3307, t3308, t3309, t3311, t3313, t3327, t3329, t3330, t3333, t3334)
}
