//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1247/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1247<F: Float>(t491: F, t5747: F, t16937: F, t28484: F, t27369: F, t16941: F, t28494: F, t7908: F, t28461: F, t7904: F, t1014: F, t28528: F) -> (F, F, F, F, F, F) {
    let t98470 = t5747 * t491;
    let t98487 = t16937 * t28484;
    let t98489 = F::new(0.20612155671296296296e-4) * t27369 * t98487;
    let t98491 = t7908 * t16941 * t28494;
    let t98519 = F::new(0.46336805555555555556e-3) * t28461 * t7904;
    let t98522 = t1014 * t28528;
    (t98470, t98487, t98489, t98491, t98519, t98522)
}
