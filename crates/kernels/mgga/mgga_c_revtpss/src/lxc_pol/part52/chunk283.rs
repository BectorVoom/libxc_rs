//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 283/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk283<F: Float>(t1357: F, t1358: F, t689: F, t556: F, t786: F, t561: F, t72: F, t686: F) -> (F, F, F, F, F) {
    let t1359 = t1357 * t1358;
    let t1361 = 0.54878743191129263322e-2 * t689 * t1359;
    let t1362 = t786 * t556;
    let t1363 = t561 * t72;
    let t1364 = t1363 * t686;
    (t1359, t1361, t1362, t1363, t1364)
}
