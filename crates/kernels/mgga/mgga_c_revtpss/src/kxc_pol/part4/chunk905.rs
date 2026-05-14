//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 905/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk905<F: Float>(t1419: F, t785: F, t1358: F, t2439: F, t784: F, t209: F) -> (F, F) {
    let t9640 = t785 * t1419;
    let t9641 = t9640 * t1358;
    let t9642 = t2439 * t9641;
    let t9644 = t784 * t784;
    let t9645 = 1.0 / t9644;
    let t9646 = t209 * t9645;
    (t9642, t9646)
}
