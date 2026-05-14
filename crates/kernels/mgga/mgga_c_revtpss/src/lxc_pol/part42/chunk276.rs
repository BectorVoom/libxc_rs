//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 276/1363 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk276<F: Float>(t837: F, t879: F, t234: F, t860: F, t213: F, t820: F, t873: F, t878: F) -> (F,) {
    let t880 = t879 * t837;
    let t883 = t234 * t860;
    let t886 = -t873 + t878 - 0.65854491829355115987e0 * t820 * t880 + 0.65854491829355115987e0 * t213 * t883;
    (t886,)
}
