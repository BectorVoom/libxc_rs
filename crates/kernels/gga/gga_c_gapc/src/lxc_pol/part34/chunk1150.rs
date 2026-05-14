//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1150/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1150<F: Float>(t35388: F, t35390: F, t35393: F, t35395: F, t35400: F, t35404: F, t35406: F, t35409: F, t35412: F, t35397: F, t36332: F, t36333: F, t36334: F, t35415: F, t35419: F, t35429: F) -> (F, F, F, F) {
    let t36335 = 0.17379648562707520765e-3 * t35388;
    let t36336 = 0.17379648562707520765e-2 * t35390;
    let t36337 = 0.45552534985326748556e-4 * t35393;
    let t36338 = 0.6951859425083008306e-3 * t35395;
    let t36340 = 0.6951859425083008306e-3 * t35400;
    let t36341 = 0.17379648562707520765e-3 * t35404;
    let t36342 = 0.14024275817241799902e-4 * t35406;
    let t36343 = 0.2530696388073708253e-5 * t35409;
    let t36344 = 0.14762395597096631476e-5 * t35412;
    let t36345 = t36332 - t36333 - t36334 - t36335 - t36336 - t36337 - t36338 + 0.53949325746737929042e-3 * t35397 - t36340 - t36341 + t36342 - t36343 - t36344;
    let t36346 = 0.11948508386861420526e-3 * t35415;
    let t36347 = 0.10862280351692200478e-4 * t35419;
    let t36349 = 0.46202407745913005506e-6 * t35429;
    (t36345, t36346, t36347, t36349)
}
