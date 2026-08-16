//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 386/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk386<F: Float>(t2439: F, t2446: F, t157: F, t182: F, t676: F, t724: F, t164: F, t723: F, t159: F, t730: F, t731: F, t2388: F, t2391: F, t2394: F, t2398: F, t2400: F, t2403: F) -> (F, F, F, F, F, F, F) {
    let t2447 = t2439 + t2446;
    let t2448 = t2447 * t157;
    let t2450 = F::cast_from(0.19751673498613801407e-1_f64) * t2448 * t182;
    let t2454 = t676 * t724;
    let t2458 = t723 * t164;
    let t2459 = F::cast_from(1.0_f64) / t2458;
    let t2460 = t159 * t2459;
    let t2461 = t730 * t730;
    let t2462 = t2461 * t731;
    let t2471 = -F::cast_from(0.78438333333333333333e0_f64) * t2388 + F::cast_from(0.15687666666666666667e1_f64) * t2391 + F::cast_from(0.68863333333333333333e0_f64) * t2394 + F::cast_from(0.14025833333333333333e0_f64) * t2398 + F::cast_from(0.28051666666666666667e0_f64) * t2400 + F::cast_from(0.17365833333333333333e0_f64) * t2403;
    (t2447, t2450, t2454, t2460, t2461, t2462, t2471)
}
