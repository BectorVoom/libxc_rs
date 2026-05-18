//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1237/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1237<F: Float>(t12987: F, t2138: F, t13036: F, t13038: F, t13040: F, t26842: F, t12808: F, t29096: F, t12898: F, t2139: F, t12851: F, t2134: F, sigma2: F) -> (F, F, F, F, F, F) {
    let t97193 = t12987 * t2138;
    let t97211 = t13036 * t13038 * sigma2 * t13040;
    let t97215 = t13036 * t26842 * t13040;
    let t97261 = t12808 * t29096;
    let t97272 = F::new(0.1270341277572436651e-3) * t2139 * t12898;
    let t97296 = F::new(5.0) / F::new(1296.0) * t2134 * t12851;
    (t97193, t97211, t97215, t97261, t97272, t97296)
}
