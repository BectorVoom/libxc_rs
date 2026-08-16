//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1995/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1995<F: Float>(t198: F, t206: F, t7086: F, t25373: F, t25392: F, t25386: F, t268: F, t41040: F, t837: F, t25372: F, t25287: F, t786: F, t789: F) -> (F, F, F, F, F, F) {
    let t92819 = t198 * t206 * t7086;
    let t92837 = t25373 * t25392;
    let t92838 = t25386 * t92837;
    let t92840 = t268 * t41040 * t837;
    let t92841 = t92838 * t92840;
    let t92843 = t25372 * t92837;
    let t92844 = t92843 * t92840;
    let t92847 = t786 * t25287 * t789;
    (t92819, t92838, t92841, t92843, t92844, t92847)
}
