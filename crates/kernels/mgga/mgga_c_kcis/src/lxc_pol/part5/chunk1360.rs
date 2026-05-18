//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1360/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1360<F: Float>(t16622: F, t4291: F, t6012: F, t17412: F, t5932: F, t2047: F, t5999: F, t17504: F, t576: F, t5905: F, t20925: F, t4261: F) -> (F, F, F, F, F) {
    let t22393 = t16622 * t4291;
    let t22394 = t22393 * t6012;
    let t22396 = t17412 * t5932;
    let t22398 = t5999 * t2047;
    let t22400 = t576 * t17504;
    let t22401 = t22400 * t5905;
    let t22403 = t4261 * t20925;
    (t22394, t22396, t22398, t22401, t22403)
}
