//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 440/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk440<F: Float>(t2254: F, t260: F, t751: F, t786: F, t2299: F, t154: F, t2141: F, t2320: F, t2324: F, t2331: F, t2338: F, t2342: F, t2387: F, t2389: F, t2394: F, t276: F, t299: F, t311: F, t837: F, t841: F, t845: F, t869: F, t871: F, t872: F) -> F {
    let t2395 = t260 * t2254;
    let t2398 = t751 * t786;
    let t2401 = t260 * t2299;
    let t2404 = -F::cast_from(0.47803703703703703703e-2_f64) * t154 * t2320 * t276 + F::cast_from(0.28682222222222222222e-1_f64) * t154 * t2324 * t276 - F::cast_from(0.28682222222222222222e-1_f64) * t154 * t837 * t845 + F::cast_from(0.21511666666666666667e-1_f64) * t154 * t2331 * t276 - F::cast_from(0.43023333333333333334e-1_f64) * t154 * t841 * t845 + F::cast_from(0.43023333333333333334e-1_f64) * t154 * t299 * t2338 - F::cast_from(0.21511666666666666667e-1_f64) * t154 * t299 * t2342 - t2387 * t260 + F::cast_from(2.0_f64) * t2389 * t872 - F::cast_from(2.0_f64) * t869 * t751 - F::cast_from(2.0_f64) * t2394 * t2395 + F::cast_from(2.0_f64) * t871 * t2398 + t871 * t2401 - t311 * t2141;
    t2404
}
