//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 709/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk709<F: Float>(t12712: F, t471: F, t1248: F, t3604: F, t1448: F, t1868: F, t197: F, t531: F, t2013: F) -> (F, F, F, F, F) {
    let t21028 = t12712 * t471;
    let t21119 = t3604 * t1248;
    let t22496 = t1868 * t1448;
    let t25081 = t197 * t531;
    let t25082 = t2013 * t25081;
    (t21028, t21119, t22496, t25081, t25082)
}
