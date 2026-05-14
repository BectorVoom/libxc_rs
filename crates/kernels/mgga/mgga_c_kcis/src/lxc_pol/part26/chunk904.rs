//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 904/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk904<F: Float>(t1517: F, t22503: F, t833: F, t1153: F, t1478: F, t1483: F, t1507: F, t17540: F, t17613: F, t1991: F, t1995: F, t2018: F, t22035: F, t22127: F, t22498: F, t2429: F, t5482: F, t5494: F, t5527: F, t562: F, t5938: F, t7233: F, t7241: F, t7245: F, t7249: F, t7341: F) -> (F,) {
    let t22505 = t1517 * t22503 * t833;
    let t22509 = 0.123825e-1 * t1507 * t7245 - 0.619125e-2 * t1507 * t7249 - 0.619125e-2 * t562 * t22035 + 0.1857375e-1 * t2018 * t5482 - 0.123825e-1 * t5938 * t1995 - 0.123825e-1 * t2018 * t5527 + 0.46434375e-2 * t1507 * t7233 + 0.9286875e-2 * t1507 * t7241 + 0.9286875e-2 * t562 * t22127 + 0.9286875e-2 * t7341 * t1478 - 0.619125e-2 * t7341 * t1483 + 0.1857375e-1 * t5938 * t1991 + 0.10612444444444444444e0 * t2429 * t22498 - 0.1857375e-1 * t17613 * t5494 - 0.26531111111111111111e-1 * t1153 * t22505 - 0.35374814814814814815e-1 * t17540;
    (t22509,)
}
