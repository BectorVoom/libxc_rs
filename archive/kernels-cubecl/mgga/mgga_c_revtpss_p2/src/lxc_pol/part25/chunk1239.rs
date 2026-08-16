//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1239/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1239<F: Float>(t2482: F, t596: F, t7036: F, t2487: F, t10820: F, t7045: F, t10863: F, t25262: F, t10828: F, t7038: F, t10832: F, t25245: F) -> (F, F, F, F, F) {
    let t93034 = t2482 * t7036 * t596;
    let t93035 = t93034 * t2487;
    let t93037 = t7045 * t10820;
    let t93039 = t25262 * t10863;
    let t93041 = t7038 * t10828;
    let t93043 = t25245 * t10832;
    (t93035, t93037, t93039, t93041, t93043)
}
