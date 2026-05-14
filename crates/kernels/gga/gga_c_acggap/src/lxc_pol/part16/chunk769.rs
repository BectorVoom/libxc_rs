//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 769/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk769<F: Float>(t1801: F, t2041: F, t1805: F, t1788: F, t7332: F, t1809: F, t570: F, t1797: F, t1784: F, t1886: F, t2001: F, t1881: F, t1844: F, t599: F, t1181: F, t2068: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t9739 = t2041 * t1801;
    let t9741 = t2041 * t1805;
    let t9743 = t7332 * t1788;
    let t9747 = t570 * t1809;
    let t9749 = t570 * t1797;
    let t9751 = t570 * t1784;
    let t9753 = t2001 * t1886;
    let t9755 = t2001 * t1881;
    let t9757 = t599 * t1844;
    let t9758 = t1181 * t9757;
    let t9759 = t2068 * t9758;
    (t9739, t9741, t9743, t9747, t9749, t9751, t9753, t9755, t9757, t9758, t9759)
}
