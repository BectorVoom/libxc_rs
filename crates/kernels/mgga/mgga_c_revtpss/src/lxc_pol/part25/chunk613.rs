//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 613/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk613<F: Float>(t1156: F, t1160: F, t1159: F, t431: F, t426: F, t1168: F, t1169: F, t3356: F, t3413: F, t3358: F, t3365: F, t3370: F, t3374: F, t3392: F, t3400: F, t3408: F, t3410: F, t3415: F, t3419: F, t3422: F, t3425: F) -> (F, F, F, F, F, F) {
    let t3447 = t1156 * t1160;
    let t3450 = t1159 * t431;
    let t3451 = F::new(1.0) / t3450;
    let t3452 = t426 * t3451;
    let t3453 = t1168 * t1168;
    let t3454 = t3453 * t1169;
    let t3459 = F::cast_from(0.68863333333333333333e0_f64) * t3356;
    let t3466 = F::cast_from(0.17365833333333333333e0_f64) * t3413;
    let t3471 = -F::new(0.17648625e1) * t3392 + F::new(0.3529725e1) * t3400 + t3459 - F::cast_from(0.34431666666666666666e0_f64) * t3358 - F::cast_from(0.34431666666666666667e0_f64) * t3365 + F::new(0.103295e1) * t3370 + F::new(0.516475e0) * t3374 + F::new(0.31558125e0) * t3408 + F::new(0.6311625e0) * t3410 + t3466 - F::cast_from(0.13892666666666666667e0_f64) * t3415 - F::cast_from(0.34731666666666666667e-1_f64) * t3419 + F::new(0.20839e0) * t3422 + F::new(0.104195e0) * t3425;
    (t3447, t3451, t3452, t3453, t3454, t3471)
}
