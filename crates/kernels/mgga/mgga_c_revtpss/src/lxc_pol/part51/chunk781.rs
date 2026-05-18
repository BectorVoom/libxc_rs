//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 781/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk781<F: Float>(t1045: F, t999: F, t1043: F, t3155: F, t12131: F, t357: F, t1448: F, t1868: F, t197: F, t531: F, t2013: F) -> (F, F, F, F, F, F) {
    let t19620 = t1045 * t999;
    let t19634 = t3155 * t1043;
    let t19639 = t12131 * t357;
    let t22496 = t1868 * t1448;
    let t25081 = t197 * t531;
    let t25082 = t2013 * t25081;
    (t19620, t19634, t19639, t22496, t25081, t25082)
}
