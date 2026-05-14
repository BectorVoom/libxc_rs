//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 874/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk874<F: Float>(t17636: F, t1962: F, t2029: F, t318: F, t86: F, t238: F, t5992: F, t1153: F, t12361: F, t12431: F, t15990: F, t15996: F, t16412: F, t16422: F, t16426: F, t17605: F, t17610: F, t17613: F, t17627: F, t17630: F, t17633: F, t3788: F, t4202: F, t545: F, t5464: F, t5494: F, t5499: F, t5947: F, t5958: F) -> (F,) {
    let t17637 = t1962 * t17636;
    let t17641 = t86 * t318 * t2029;
    let t17645 = 0.53062222222222222222e-1 * t86 * t238 * t5992;
    let t17648 = -0.26531111111111111111e-1 * t1153 * t17605 - 0.44218518518518518518e-1 * t1153 * t17610 - 0.1857375e-1 * t17613 * t3788 - 0.1857375e-1 * t4202 * t16422 - 0.1857375e-1 * t12361 * t5494 + 0.123825e-1 * t5958 * t15990 + 0.11791604938271604938e-1 * t12431 - 0.46434375e-2 * t5947 * t15996 - 0.1857375e-1 * t12361 * t5464 - 0.232171875e-2 * t17627 * t16412 + 0.619125e-2 * t17630 * t545 + 0.24765e-1 * t17633 * t5499 + 0.371475e-1 * t4202 * t17637 + 0.88437037037037037037e-2 * t17641 - t17645 - 0.9286875e-2 * t4202 * t16426;
    (t17648,)
}
