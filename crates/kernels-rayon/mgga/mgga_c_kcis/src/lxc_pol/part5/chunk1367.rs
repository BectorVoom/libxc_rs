//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1367/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1367(t1517: f64, t22503: f64, t833: f64, t1153: f64, t1478: f64, t1483: f64, t1507: f64, t17540: f64, t17613: f64, t1991: f64, t1995: f64, t2018: f64, t22035: f64, t22127: f64, t22498: f64, t2429: f64, t5482: f64, t5494: f64, t5527: f64, t562: f64, t5938: f64, t7233: f64, t7241: f64, t7245: f64, t7249: f64, t7341: f64) -> f64 {
    let t22505 = t1517 * t22503 * t833;
    let t22509 = 0.123825e-1_f64 * t1507 * t7245 - 0.619125e-2_f64 * t1507 * t7249 - 0.619125e-2_f64 * t562 * t22035 + 0.1857375e-1_f64 * t2018 * t5482 - 0.123825e-1_f64 * t5938 * t1995 - 0.123825e-1_f64 * t2018 * t5527 + 0.46434375e-2_f64 * t1507 * t7233 + 0.9286875e-2_f64 * t1507 * t7241 + 0.9286875e-2_f64 * t562 * t22127 + 0.9286875e-2_f64 * t7341 * t1478 - 0.619125e-2_f64 * t7341 * t1483 + 0.1857375e-1_f64 * t5938 * t1991 + 0.10612444444444444444e0_f64 * t2429 * t22498 - 0.1857375e-1_f64 * t17613 * t5494 - 0.26531111111111111111e-1_f64 * t1153 * t22505 - 0.35374814814814814815e-1_f64 * t17540;
    t22509
}
