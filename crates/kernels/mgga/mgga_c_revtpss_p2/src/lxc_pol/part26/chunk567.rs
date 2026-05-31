//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 567/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk567<F: Float>(t1188: F, t3497: F, t3356: F, t3413: F, t3358: F, t3365: F, t3370: F, t3374: F, t3392: F, t3400: F, t3408: F, t3410: F, t3415: F, t3419: F, t3422: F, t3425: F) -> (F, F) {
    let t3498 = t3497 * t1188;
    let t3503 = F::cast_from(0.40256666666666666667e0_f64) * t3356;
    let t3510 = F::cast_from(0.137975e0_f64) * t3413;
    let t3515 = -F::cast_from(0.1294625e1_f64) * t3392 + F::cast_from(0.258925e1_f64) * t3400 + t3503 - F::cast_from(0.20128333333333333334e0_f64) * t3358 - F::cast_from(0.20128333333333333333e0_f64) * t3365 + F::cast_from(0.60385e0_f64) * t3370 + F::cast_from(0.301925e0_f64) * t3374 + F::cast_from(0.82524375e-1_f64) * t3408 + F::cast_from(0.16504875e0_f64) * t3410 + t3510 - F::cast_from(0.11038e0_f64) * t3415 - F::cast_from(0.27595e-1_f64) * t3419 + F::cast_from(0.16557e0_f64) * t3422 + F::cast_from(0.82785e-1_f64) * t3425;
    (t3498, t3515)
}
