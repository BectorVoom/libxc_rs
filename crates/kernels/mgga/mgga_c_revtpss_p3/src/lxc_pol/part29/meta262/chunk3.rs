//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1079/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1079<F: Float>(t30: F, t890: F, t33: F, t775: F, t1315: F, t196: F, t197: F) -> (F, F, F, F, F) {
    let t7092 = t30 * t890;
    let t7200 = t33 * t775;
    let t7207 = t33 * t890;
    let t7234 = t1315 * t196;
    let t7235 = t7234 * t197;
    (t7092, t7200, t7207, t7234, t7235)
}
