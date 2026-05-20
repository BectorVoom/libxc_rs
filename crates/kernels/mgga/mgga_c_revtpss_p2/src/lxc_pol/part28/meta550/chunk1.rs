//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2000/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2000<F: Float>(t1926: F, t92576: F, t1927: F, t2315: F, t2247: F, t2259: F, t2269: F, t48: F, t2275: F, t613: F, t10355: F, t43: F) -> (F, F, F, F, F, F) {
    let t92577 = t1926 * t92576;
    let t92584 = t1927 * t2315;
    let t92585 = t1926 * t92584;
    let t92588 = t2247 * t2259;
    let t92597 = t2269 * t48;
    let t92600 = t613 * t2275;
    let t92605 = t43 * t10355;
    (t92577, t92585, t92588, t92597, t92600, t92605)
}
