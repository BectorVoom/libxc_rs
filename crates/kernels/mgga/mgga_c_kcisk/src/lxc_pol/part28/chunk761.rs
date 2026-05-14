//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 761/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk761<F: Float>(t9379: F, t9382: F, t9368: F, t9326: F, t9329: F, t9333: F, t9362: F, t9369: F, t9373: F, t9377: F, t1139: F, t2701: F, t1147: F, t2705: F, t9343: F, t9346: F, t9348: F, t9350: F, t9353: F, t9356: F) -> (F, F, F, F) {
    let t9383 = t9379 * t9382;
    let t9385 = t9379 * t9368;
    let t9390 = -0.10416666666666666667e-1 * t9362 + 0.40208333333333333335e-2 * t9369 - 0.10416666666666666667e-1 * t9373 + 0.24305555555555555556e-1 * t9377 + 0.10416666666666666667e-1 * t9383 + 0.10416666666666666667e-1 * t9385 - 0.92858888888888888886e-2 * t9326 + 0.69644166666666666665e-2 * t9329 - 0.69644166666666666665e-2 * t9333;
    let t9392 = t2701 * t1139;
    let t9395 = t2705 * t1147;
    let t9404 = 0.1875e0 * t9343 - 0.1875e0 * t9346 - 0.375e0 * t9348 - 0.809375e-1 * t9350 + 0.809375e-1 * t9353 + 0.32375e0 * t9356;
    (t9390, t9392, t9395, t9404)
}
