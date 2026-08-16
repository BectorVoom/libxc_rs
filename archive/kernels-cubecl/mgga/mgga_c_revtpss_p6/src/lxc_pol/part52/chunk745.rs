//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 745/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk745<F: Float>(t225: F, t7759: F, t1568: F, t1955: F, t1579: F, t1949: F, t7071: F, t1558: F, t231: F, t7076: F, t233: F, t1957: F) -> (F, F, F, F, F, F, F, F) {
    let t7760 = t7759 * t225;
    let t7766 = t1955 * t1568;
    let t7769 = t1949 * t1579;
    let t7770 = t7071 * t7769;
    let t7774 = t1949 * t1558 * t231;
    let t7775 = t7076 * t7774;
    let t7778 = t233 * t7759;
    let t7779 = t1957 * t7778;
    (t7760, t7766, t7769, t7770, t7774, t7775, t7778, t7779)
}
