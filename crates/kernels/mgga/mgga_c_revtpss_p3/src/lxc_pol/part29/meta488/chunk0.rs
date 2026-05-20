//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1772/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1772<F: Float>(t1450: F, t5591: F, t2013: F, t8995: F, t1448: F, t1907: F, t4292: F, t93: F, t2106: F, t9593: F) -> (F, F, F, F, F) {
    let t28176 = t1450 * t5591;
    let t28196 = t2013 * t8995;
    let t28198 = t1907 * t1448;
    let t28219 = t93 * t4292;
    let t28286 = t2106 * t9593;
    (t28176, t28196, t28198, t28219, t28286)
}
