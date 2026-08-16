//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2002/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2002<F: Float>(t10301: F, t25105: F, t116: F, t25168: F, t1962: F, t41154: F, t2411: F, t25435: F, t605: F, t198: F, t206: F, t7086: F) -> (F, F, F, F, F, F) {
    let t92702 = t10301 * t25105;
    let t92737 = t25168 * t116;
    let t92742 = t1962 * t41154;
    let t92775 = t25435 * t2411;
    let t92790 = t2411 * t605;
    let t92819 = t198 * t206 * t7086;
    (t92702, t92737, t92742, t92775, t92790, t92819)
}
