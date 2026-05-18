//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1116/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1116<F: Float>(t239: F, t25260: F, t820: F, t2726: F, t7036: F, t843: F, t839: F, t241: F) -> (F, F, F, F, F) {
    let t25262 = t820 * t25260 * t239;
    let t25263 = t25262 * t2726;
    let t25266 = t820 * t7036 * t843;
    let t25267 = t25266 * t839;
    let t25270 = t820 * t7036 * t241;
    (t25262, t25263, t25266, t25267, t25270)
}
