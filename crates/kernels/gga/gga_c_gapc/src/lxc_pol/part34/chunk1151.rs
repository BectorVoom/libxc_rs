//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1151/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1151<F: Float>(t35432: F, t35435: F, t35439: F, t35443: F, t35447: F, t35449: F, t35453: F, t35458: F, t35422: F, t35451: F, t36346: F, t36347: F, t36349: F, t35463: F, t35466: F, t35471: F) -> (F, F, F, F) {
    let t36350 = 0.10862280351692200478e-4 * t35432;
    let t36351 = 0.10862280351692200478e-4 * t35435;
    let t36352 = 0.64377114884362441502e-6 * t35439;
    let t36353 = 0.16871309253824721687e-5 * t35443;
    let t36354 = 0.16871309253824721687e-5 * t35447;
    let t36355 = 0.45552534985326748556e-4 * t35449;
    let t36357 = 0.6951859425083008306e-3 * t35453;
    let t36358 = 0.14762395597096631476e-5 * t35458;
    let t36359 = t36346 + t36347 - 0.54311401758461002391e-5 * t35422 + t36349 - t36350 - t36351 - t36352 - t36353 - t36354 + t36355 - 0.49106559089941822995e-4 * t35451 + t36357 + t36358;
    let t36361 = 0.36207601172307334926e-6 * t35463;
    let t36362 = 0.36207601172307334926e-6 * t35466;
    let t36363 = 0.79204127564422295151e-7 * t35471;
    (t36359, t36361, t36362, t36363)
}
