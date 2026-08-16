//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1368/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1368(t1507: f64, t543: f64, t1319: f64, t1419: f64, t1962: f64, t2029: f64, t318: f64, t86: f64, t238: f64, t5992: f64, t1153: f64, t12361: f64, t12431: f64, t15990: f64, t15996: f64, t16412: f64, t16422: f64, t16426: f64, t17605: f64, t17610: f64, t17613: f64, t17627: f64, t17630: f64, t3788: f64, t4202: f64, t545: f64, t5464: f64, t5494: f64, t5499: f64, t5947: f64, t5958: f64) -> (f64, f64) {
    let t17633 = t1507 * t543;
    let t17636 = t1319 * t1419;
    let t17637 = t1962 * t17636;
    let t17641 = t86 * t318 * t2029;
    let t17645 = 0.53062222222222222222e-1_f64 * t86 * t238 * t5992;
    let t17648 = -0.26531111111111111111e-1_f64 * t1153 * t17605 - 0.44218518518518518518e-1_f64 * t1153 * t17610 - 0.1857375e-1_f64 * t17613 * t3788 - 0.1857375e-1_f64 * t4202 * t16422 - 0.1857375e-1_f64 * t12361 * t5494 + 0.123825e-1_f64 * t5958 * t15990 + 0.11791604938271604938e-1_f64 * t12431 - 0.46434375e-2_f64 * t5947 * t15996 - 0.1857375e-1_f64 * t12361 * t5464 - 0.232171875e-2_f64 * t17627 * t16412 + 0.619125e-2_f64 * t17630 * t545 + 0.24765e-1_f64 * t17633 * t5499 + 0.371475e-1_f64 * t4202 * t17637 + 0.88437037037037037037e-2_f64 * t17641 - t17645 - 0.9286875e-2_f64 * t4202 * t16426;
    (t17636, t17648)
}
