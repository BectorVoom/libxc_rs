//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1079/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1079<F: Float>(t1188: F, t6518: F, t3503: F, t3510: F, t5044: F, t5093: F, t6423: F, t6427: F, t6431: F, t6443: F, t6450: F, t6456: F, t6458: F, t6462: F, t6465: F, t6468: F) -> (F, F) {
    let t6519 = t6518 * t1188;
    let t6534 = -F::cast_from(0.1294625e1_f64) * t6443 + F::cast_from(0.258925e1_f64) * t6450 + t3503 - F::cast_from(0.20128333333333333334e0_f64) * t5044 - F::cast_from(0.20128333333333333333e0_f64) * t6423 + F::cast_from(0.60385e0_f64) * t6427 + F::cast_from(0.301925e0_f64) * t6431 + F::cast_from(0.82524375e-1_f64) * t6456 + F::cast_from(0.16504875e0_f64) * t6458 + t3510 - F::cast_from(0.11038e0_f64) * t5093 - F::cast_from(0.27595e-1_f64) * t6462 + F::cast_from(0.16557e0_f64) * t6465 + F::cast_from(0.82785e-1_f64) * t6468;
    (t6519, t6534)
}
