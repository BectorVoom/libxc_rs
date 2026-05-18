//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 445/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk445<F: Float>(t2516: F, t760: F, t675: F, t681: F, t268: F, t702: F) -> (F, F, F) {
    let t2518 = F::new(0.5848223622634646207e0) * t760 * t2516;
    let t2519 = t675 * t681;
    let t2522 = F::new(0.35616666666666666666e-1) * t268 * t2519 * t702;
    (t2518, t2519, t2522)
}
