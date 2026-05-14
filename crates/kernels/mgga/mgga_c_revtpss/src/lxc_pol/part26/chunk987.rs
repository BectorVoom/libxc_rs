//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 987/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk987<F: Float>(t2430: F, t890: F, t10259: F, t93: F, t10301: F, t607: F, t10309: F, t1927: F, t2248: F, t644: F, t6977: F, t25113: F, t77: F, t2315: F, t2247: F, t2259: F) -> (F, F, F, F, F, F, F, F, F) {
    let t51806 = t2430 * t890;
    let t60551 = t93 * t10259;
    let t92565 = t10301 * t607;
    let t92568 = t10309 * t607;
    let t92569 = t1927 * t2248;
    let t92576 = t6977 * t644;
    let t92581 = t77 * t25113 * t644;
    let t92584 = t1927 * t2315;
    let t92588 = t2247 * t2259;
    (t51806, t60551, t92565, t92568, t92569, t92576, t92581, t92584, t92588)
}
