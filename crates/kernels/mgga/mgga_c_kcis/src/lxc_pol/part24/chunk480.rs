//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 480/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk480<F: Float>(t3463: F, t1094: F, t1164: F, t3177: F, t381: F, t1242: F, t1247: F, t1241: F, t68: F, sigma0: F) -> (F, F, F, F, F, F) {
    let t3464 = t3463 * sigma0;
    let t3473 = t1164 * t1094;
    let t3474 = t3473 * sigma0;
    let t3477 = t3177 * t381;
    let t3487 = t1242 * t1247;
    let t3489 = t1241 * t68;
    (t3464, t3473, t3474, t3477, t3487, t3489)
}
