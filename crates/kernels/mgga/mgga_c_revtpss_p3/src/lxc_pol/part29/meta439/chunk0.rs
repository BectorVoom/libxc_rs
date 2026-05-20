//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1649/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1649<F: Float>(t4003: F, t5658: F, t1448: F, t1868: F, t197: F, t531: F, t2013: F) -> (F, F, F, F) {
    let t21990 = t4003 * t5658;
    let t22496 = t1868 * t1448;
    let t25081 = t197 * t531;
    let t25082 = t2013 * t25081;
    (t21990, t22496, t25081, t25082)
}
