//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 243/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk243<F: Float>(t315: F, t964: F, t902: F, t928: F, t323: F) -> (F, F, F, F) {
    let t965 = t315 * t964;
    let t967 = F::new(0.301925e0) * t902;
    let t970 = F::new(0.82785e-1) * t928;
    let t973 = F::new(1.0) / t323;
    (t965, t967, t970, t973)
}
