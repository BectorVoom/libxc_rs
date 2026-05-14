//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 906/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk906<F: Float>(t1814: F, t5089: F, t1644: F, t6799: F, t2368: F, t4741: F, t15993: F, t2378: F, t2877: F, t22: F, t5815: F, t6831: F, t6828: F, t827: F, t6825: F, t15991: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t16303 = t5089 * t1814;
    let t16351 = t6799 * t1644;
    let t16356 = t2368 * t4741;
    let t16379 = 0.39862222222222222222e0 * t15993;
    let t16389 = t2877 * t2378;
    let t16391 = t22 * t5815;
    let t16392 = t16391 * t6831;
    let t16398 = t827 * t6828;
    let t16399 = 0.21908444444444444444e0 * t16398;
    let t16400 = t827 * t6825;
    let t16447 = 0.41203703703703703704e-2 * t15991;
    (t16303, t16351, t16356, t16379, t16389, t16392, t16398, t16399, t16400, t16447)
}
