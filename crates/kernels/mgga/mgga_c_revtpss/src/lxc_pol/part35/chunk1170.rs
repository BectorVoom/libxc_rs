//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1170/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1170<F: Float>(t2411: F, t30419: F, t105936: F, t95822: F, t212: F, t30379: F, t689: F, t780: F, t95537: F, t213: F, t30410: F, t686: F, t72: F) -> (F, F, F, F, F, F) {
    let t110177 = t30419 * t2411;
    let t110236 = t95822 * t105936;
    let t110245 = t689 * t212 * t30379 * t780;
    let t110247 = t95537 * t105936;
    let t110256 = t213 * t30379;
    let t110275 = t30410 * t72 * t686;
    (t110177, t110236, t110245, t110247, t110256, t110275)
}
