//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1152/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1152<F: Float>(t35475: F, t35478: F, t35480: F, t35485: F, t35489: F, t35493: F, t35495: F, t35500: F, t35503: F, t35482: F, t36361: F, t36362: F, t36363: F, t35506: F, t35510: F, t35512: F) -> (F, F, F, F) {
    let t36364 = 0.47522476538653377092e-5 * t35475;
    let t36365 = 0.47522476538653377092e-5 * t35478;
    let t36366 = 0.5061392776147416506e-5 * t35480;
    let t36368 = 0.45552534985326748556e-4 * t35485;
    let t36369 = 0.5061392776147416506e-5 * t35489;
    let t36370 = 0.5061392776147416506e-5 * t35493;
    let t36371 = 0.2530696388073708253e-5 * t35495;
    let t36372 = 0.86898242813537603825e-4 * t35500;
    let t36373 = 0.5061392776147416506e-5 * t35503;
    let t36374 = -t36361 - t36362 - t36363 + t36364 + t36365 - t36366 + 0.42242201367691890748e-5 * t35482 - t36368 - t36369 + t36370 + t36371 - t36372 + t36373;
    let t36375 = 0.2530696388073708253e-5 * t35506;
    let t36376 = 0.14762395597096631476e-5 * t35510;
    let t36377 = 0.17379648562707520765e-3 * t35512;
    (t36374, t36375, t36376, t36377)
}
