//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1353/1427 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1353<F: Float>(t35415: F, t35419: F, t35429: F, t35432: F, t35435: F, t35439: F, t35443: F, t35447: F, t35449: F, t35453: F, t35458: F, t35422: F, t35451: F) -> F {
    let t36346 = F::new(0.11948508386861420526e-3) * t35415;
    let t36347 = F::new(0.10862280351692200478e-4) * t35419;
    let t36349 = F::new(0.46202407745913005506e-6) * t35429;
    let t36350 = F::new(0.10862280351692200478e-4) * t35432;
    let t36351 = F::new(0.10862280351692200478e-4) * t35435;
    let t36352 = F::new(0.64377114884362441502e-6) * t35439;
    let t36353 = F::new(0.16871309253824721687e-5) * t35443;
    let t36354 = F::new(0.16871309253824721687e-5) * t35447;
    let t36355 = F::new(0.45552534985326748556e-4) * t35449;
    let t36357 = F::new(0.6951859425083008306e-3) * t35453;
    let t36358 = F::new(0.14762395597096631476e-5) * t35458;
    let t36359 = t36346 + t36347 - F::new(0.54311401758461002391e-5) * t35422 + t36349 - t36350 - t36351 - t36352 - t36353 - t36354 + t36355 - F::new(0.49106559089941822995e-4) * t35451 + t36357 + t36358;
    t36359
}
