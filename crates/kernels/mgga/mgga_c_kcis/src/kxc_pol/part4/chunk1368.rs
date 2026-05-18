//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1368/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1368<F: Float>(t1507: F, t543: F, t1319: F, t1419: F, t1962: F, t2029: F, t318: F, t86: F, t238: F, t5992: F, t1153: F, t12361: F, t12431: F, t15990: F, t15996: F, t16412: F, t16422: F, t16426: F, t17605: F, t17610: F, t17613: F, t17627: F, t17630: F, t3788: F, t4202: F, t545: F, t5464: F, t5494: F, t5499: F, t5947: F, t5958: F) -> (F, F) {
    let t17633 = t1507 * t543;
    let t17636 = t1319 * t1419;
    let t17637 = t1962 * t17636;
    let t17641 = t86 * t318 * t2029;
    let t17645 = F::new(0.53062222222222222222e-1) * t86 * t238 * t5992;
    let t17648 = -F::new(0.26531111111111111111e-1) * t1153 * t17605 - F::new(0.44218518518518518518e-1) * t1153 * t17610 - F::new(0.1857375e-1) * t17613 * t3788 - F::new(0.1857375e-1) * t4202 * t16422 - F::new(0.1857375e-1) * t12361 * t5494 + F::new(0.123825e-1) * t5958 * t15990 + F::new(0.11791604938271604938e-1) * t12431 - F::new(0.46434375e-2) * t5947 * t15996 - F::new(0.1857375e-1) * t12361 * t5464 - F::new(0.232171875e-2) * t17627 * t16412 + F::new(0.619125e-2) * t17630 * t545 + F::new(0.24765e-1) * t17633 * t5499 + F::new(0.371475e-1) * t4202 * t17637 + F::new(0.88437037037037037037e-2) * t17641 - t17645 - F::new(0.9286875e-2) * t4202 * t16426;
    (t17636, t17648)
}
