//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 973/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk973<F: Float>(t11485: F, t11500: F, t973: F, t3010: F, t963: F, t315: F, t3013: F, t323: F, t11467: F, t2962: F, t955: F, t2970: F, t953: F) -> (F, F, F, F, F, F, F, F) {
    let t11501 = t11485 + t11500;
    let t11502 = t11501 * t973;
    let t11506 = F::new(1.0) / t3010 / t963;
    let t11507 = t315 * t11506;
    let t11509 = F::new(1.0) / t3013 / t323;
    let t11510 = t11467 * t11509;
    let t11513 = t955 * t2962;
    let t11517 = t2962 * t2970 * t953;
    (t11501, t11502, t11506, t11507, t11509, t11510, t11513, t11517)
}
