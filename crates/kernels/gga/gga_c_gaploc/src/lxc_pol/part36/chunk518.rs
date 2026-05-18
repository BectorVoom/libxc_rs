//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 518/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk518<F: Float>(t1: F, t3116: F, t106: F, t192: F, t1537: F, t2386: F, t4511: F, t4781: F, t536: F, t567: F, t597: F, t9351: F, t9355: F, t9359: F, t9363: F, t9366: F, t9370: F, t9371: F, t9374: F, t9377: F, t9380: F) -> F {
    let t9383 = t3116 * t1;
    let t9384 = t9383 * t106;
    let t9385 = t9384 * t192;
    let t9388 = -F::new(0.10725146985555128001e1) * t9351 * t2386 + F::new(0.15337170381568299871e1) * t4781 * t9355 - F::new(0.51123901271894332902e0) * t1537 * t9359 + t9363 + t9366 - t9370 - F::new(0.7988109573733489516e-2) * t9371 + F::new(0.69017266717057349418e1) * t4511 * t9374 + F::new(0.23005755572352449806e2) * t597 * t9377 + F::new(0.23005755572352449806e1) * t567 * t9380 + F::new(0.35750489951850426669e0) * t536 * t9385;
    t9388
}
