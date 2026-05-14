//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 887/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk887<F: Float>(t20295: F, t20437: F, t20298: F, t2115: F, t4030: F, t45: F, t6091: F, t1233: F, t6032: F, t4080: F, t13776: F, t378: F, t1305: F, t6149: F, t3924: F, t6217: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t20454 = 0.13418888888888888889e0 * t20295;
    let t20504 = 0.43816888888888888888e0 * t20437;
    let t20510 = 0.39862222222222222222e0 * t20298;
    let t20531 = 0.41203703703703703704e-2 * t20295;
    let t20532 = 0.12361111111111111111e-1 * t20298;
    let t20552 = t2115 * t4030;
    let t20557 = t45 * t6091;
    let t20562 = t6032 * t1233;
    let t20567 = t2115 * t4080;
    let t20596 = t378 * t13776;
    let t20613 = 0.35981577432354634426e-1 * t6149 * t1305;
    let t20625 = t6217 * t3924;
    (t20454, t20504, t20510, t20531, t20532, t20552, t20557, t20562, t20567, t20596, t20613, t20625)
}
