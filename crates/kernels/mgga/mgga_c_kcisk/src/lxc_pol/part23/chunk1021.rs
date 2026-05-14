//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1021/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1021<F: Float>(t1266: F, t1275: F, t20469: F, t13536: F, t13650: F, t20302: F, t20402: F, t20406: F, t20409: F, t20412: F, t20415: F, t20417: F, t20420: F, t20422: F, t20437: F, t20298: F) -> (F, F, F, F) {
    let t20471 = t1266 * t20469 * t1275;
    let t20498 = 0.99655555555555555557e-1 * t13536 + 0.10954222222222222222e0 * t13650 - 0.32862666666666666666e0 * t20402 + 0.14240488888888888888e1 * t20406 + 0.21924222222222222222e1 * t20302 + 0.1898925e1 * t20409 + 0.142419375e1 * t20412 - 0.76790625e-1 * t20415 - 0.9494625e0 * t20417 + 0.3071625e0 * t20420 + 0.15358125e0 * t20422;
    let t20504 = 0.43816888888888888888e0 * t20437;
    let t20510 = 0.39862222222222222222e0 * t20298;
    (t20471, t20498, t20504, t20510)
}
