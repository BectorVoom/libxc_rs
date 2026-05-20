//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1978/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1978<F: Float>(t10309: F, t25105: F, t45972: F, t6957: F, t1962: F, t41154: F, t2411: F, t605: F, t198: F, t206: F, t7086: F, t25373: F, t25392: F) -> (F, F, F, F, F, F) {
    let t92687 = t10309 * t25105;
    let t92690 = t45972 * t6957;
    let t92742 = t1962 * t41154;
    let t92790 = t2411 * t605;
    let t92819 = t198 * t206 * t7086;
    let t92837 = t25373 * t25392;
    (t92687, t92690, t92742, t92790, t92819, t92837)
}
