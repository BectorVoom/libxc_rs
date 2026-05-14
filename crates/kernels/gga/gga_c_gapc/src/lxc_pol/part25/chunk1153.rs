//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1153/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1153<F: Float>(t35393: F, t35395: F, t35400: F, t35404: F, t35406: F, t35409: F, t35412: F, t35415: F, t35419: F, t35429: F, t35432: F, t35435: F, t35439: F, t35443: F, t35447: F, t35449: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t36337 = 0.45552534985326748556e-4 * t35393;
    let t36338 = 0.6951859425083008306e-3 * t35395;
    let t36340 = 0.6951859425083008306e-3 * t35400;
    let t36341 = 0.17379648562707520765e-3 * t35404;
    let t36342 = 0.14024275817241799902e-4 * t35406;
    let t36343 = 0.2530696388073708253e-5 * t35409;
    let t36344 = 0.14762395597096631476e-5 * t35412;
    let t36346 = 0.11948508386861420526e-3 * t35415;
    let t36347 = 0.10862280351692200478e-4 * t35419;
    let t36349 = 0.46202407745913005506e-6 * t35429;
    let t36350 = 0.10862280351692200478e-4 * t35432;
    let t36351 = 0.10862280351692200478e-4 * t35435;
    let t36352 = 0.64377114884362441502e-6 * t35439;
    let t36353 = 0.16871309253824721687e-5 * t35443;
    let t36354 = 0.16871309253824721687e-5 * t35447;
    let t36355 = 0.45552534985326748556e-4 * t35449;
    (t36337, t36338, t36340, t36341, t36342, t36343, t36344, t36346, t36347, t36349, t36350, t36351, t36352, t36353, t36354, t36355)
}
