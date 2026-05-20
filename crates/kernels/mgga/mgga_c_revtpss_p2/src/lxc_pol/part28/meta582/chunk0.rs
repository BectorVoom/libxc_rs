//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2047/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2047<F: Float>(t27668: F, t995: F, t25610: F, t25460: F, t3057: F, t25698: F, t378: F, t8521: F, t25705: F, t3336: F, t11108: F, t7177: F) -> (F, F, F, F, F, F) {
    let t94080 = t995 * t27668;
    let t94085 = t25610 * t27668;
    let t94095 = t3057 * t25460;
    let t94121 = t25698 * t378;
    let t94122 = t94121 * t8521;
    let t94138 = t25705 * t3336;
    let t94142 = t7177 * t11108;
    (t94080, t94085, t94095, t94122, t94138, t94142)
}
