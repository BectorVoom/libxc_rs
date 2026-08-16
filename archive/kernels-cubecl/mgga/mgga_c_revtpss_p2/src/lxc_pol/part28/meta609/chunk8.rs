//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2127/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2127<F: Float>(t1450: F, t2014: F, t532: F, t97716: F, t97752: F, t97791: F, t97827: F, t97854: F, t97903: F, t97938: F, t97969: F, t97994: F, t98022: F, t98061: F, t98092: F, t98318: F, t98353: F, t98388: F, t98414: F) -> F {
    let t98421 = t2014 * t532 * (t97716 + t97752 + t97791 + t97827 + t97854 + t97903 + t97938 + t97969 + t97994 + t98022 + t98061 + t98092 + t98318 + t98353 + t98388 + t98414) * t1450;
    t98421
}
