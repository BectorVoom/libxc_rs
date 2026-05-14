//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 565/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk565<F: Float>(t1188: F, t3497: F, t3356: F, t3413: F, t3358: F, t3365: F, t3370: F, t3374: F, t3392: F, t3400: F, t3408: F, t3410: F, t3415: F, t3419: F, t3422: F, t3425: F) -> (F, F) {
    let t3498 = t3497 * t1188;
    let t3503 = 0.40256666666666666667e0 * t3356;
    let t3510 = 0.137975e0 * t3413;
    let t3515 = -0.1294625e1 * t3392 + 0.258925e1 * t3400 + t3503 - 0.20128333333333333334e0 * t3358 - 0.20128333333333333333e0 * t3365 + 0.60385e0 * t3370 + 0.301925e0 * t3374 + 0.82524375e-1 * t3408 + 0.16504875e0 * t3410 + t3510 - 0.11038e0 * t3415 - 0.27595e-1 * t3419 + 0.16557e0 * t3422 + 0.82785e-1 * t3425;
    (t3498, t3515)
}
