//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1355/1429 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1355<F: Float>(t35415: F, t35419: F, t35429: F, t35432: F, t35435: F, t35439: F, t35443: F, t35447: F, t35449: F, t35453: F, t35458: F, t35422: F, t35451: F) -> F {
    let t36346 = F::cast_from(0.11948508386861420526e-3_f64) * t35415;
    let t36347 = F::cast_from(0.10862280351692200478e-4_f64) * t35419;
    let t36349 = F::cast_from(0.46202407745913005506e-6_f64) * t35429;
    let t36350 = F::cast_from(0.10862280351692200478e-4_f64) * t35432;
    let t36351 = F::cast_from(0.10862280351692200478e-4_f64) * t35435;
    let t36352 = F::cast_from(0.64377114884362441502e-6_f64) * t35439;
    let t36353 = F::cast_from(0.16871309253824721687e-5_f64) * t35443;
    let t36354 = F::cast_from(0.16871309253824721687e-5_f64) * t35447;
    let t36355 = F::cast_from(0.45552534985326748556e-4_f64) * t35449;
    let t36357 = F::cast_from(0.6951859425083008306e-3_f64) * t35453;
    let t36358 = F::cast_from(0.14762395597096631476e-5_f64) * t35458;
    let t36359 = t36346 + t36347 - F::cast_from(0.54311401758461002391e-5_f64) * t35422 + t36349 - t36350 - t36351 - t36352 - t36353 - t36354 + t36355 - F::cast_from(0.49106559089941822995e-4_f64) * t35451 + t36357 + t36358;
    t36359
}
