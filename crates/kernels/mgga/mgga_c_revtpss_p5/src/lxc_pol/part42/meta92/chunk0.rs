//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 528/1505 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk528<F: Float>(t599: F, t602: F, t89: F, t90: F, t29: F, t2: F, t580: F) -> (F, F, F, F) {
    let t2242 = t599 * t602;
    let t2246 = F::new(1.0) / t90 / t89;
    let t2247 = t29 * t2246;
    let t2255 = t2 * t580;
    (t2242, t2246, t2247, t2255)
}
