//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 714/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk714<F: Float>(t2365: F, t28924: F, t6111: F, t12656: F, t22665: F, t7427: F, t29285: F, t10914: F, t28669: F, t2021: F, t7372: F, t9816: F, t28152: F, t787: F, t9824: F, t2684: F, t7354: F) -> (F, F, F, F, F, F, F, F) {
    let t41454 = t6111 * t2365 * t28924;
    let t41457 = t7427 * t22665 * t12656;
    let t41460 = t6111 * t2365 * t29285;
    let t41463 = t10914 * t2365 * t28669;
    let t41466 = t2021 * t9816 * t7372;
    let t41468 = t787 * t28152;
    let t41469 = t41468 * t9824;
    let t41474 = t2684 * t7354 * t12656;
    (t41454, t41457, t41460, t41463, t41466, t41468, t41469, t41474)
}
