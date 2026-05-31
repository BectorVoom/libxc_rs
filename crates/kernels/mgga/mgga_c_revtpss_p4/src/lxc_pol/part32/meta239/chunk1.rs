//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1014/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1014<F: Float>(t141: F, t6461: F, t1145: F, t6425: F, t6429: F, t3402: F, t3414: F, t5044: F, t5093: F, t6423: F, t6427: F, t6431: F, t6443: F, t6450: F, t6456: F, t6458: F) -> (F, F, F, F, F, F) {
    let t6462 = t141 * t6461;
    let t6464 = t1145 * t6425;
    let t6465 = t141 * t6464;
    let t6467 = t1145 * t6429;
    let t6468 = t141 * t6467;
    let t6470 = -F::cast_from(0.9494625e0_f64) * t6443 + F::cast_from(0.1898925e1_f64) * t6450 + t3402 - F::cast_from(0.19931111111111111111e0_f64) * t5044 - F::cast_from(0.19931111111111111111e0_f64) * t6423 + F::cast_from(0.59793333333333333334e0_f64) * t6427 + F::cast_from(0.29896666666666666667e0_f64) * t6431 + F::cast_from(0.15358125e0_f64) * t6456 + F::cast_from(0.3071625e0_f64) * t6458 + t3414 - F::cast_from(0.10954222222222222222e0_f64) * t5093 - F::cast_from(0.27385555555555555556e-1_f64) * t6462 + F::cast_from(0.16431333333333333333e0_f64) * t6465 + F::cast_from(0.82156666666666666667e-1_f64) * t6468;
    (t6462, t6464, t6465, t6467, t6468, t6470)
}
