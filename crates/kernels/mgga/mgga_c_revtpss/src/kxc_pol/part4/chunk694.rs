//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 694/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk694<F: Float>(t1169: F, t3453: F, t3356: F, t3413: F, t3358: F, t3365: F, t3370: F, t3374: F, t3392: F, t3400: F, t3408: F, t3410: F, t3415: F, t3419: F, t3422: F, t3425: F) -> (F, F, F, F) {
    let t3454 = t3453 * t1169;
    let t3459 = F::cast_from(0.68863333333333333333e0_f64) * t3356;
    let t3466 = F::cast_from(0.17365833333333333333e0_f64) * t3413;
    let t3471 = -F::new(0.17648625e1) * t3392 + F::new(0.3529725e1) * t3400 + t3459 - F::cast_from(0.34431666666666666666e0_f64) * t3358 - F::cast_from(0.34431666666666666667e0_f64) * t3365 + F::new(0.103295e1) * t3370 + F::new(0.516475e0) * t3374 + F::new(0.31558125e0) * t3408 + F::new(0.6311625e0) * t3410 + t3466 - F::cast_from(0.13892666666666666667e0_f64) * t3415 - F::cast_from(0.34731666666666666667e-1_f64) * t3419 + F::new(0.20839e0) * t3422 + F::new(0.104195e0) * t3425;
    (t3454, t3459, t3466, t3471)
}
