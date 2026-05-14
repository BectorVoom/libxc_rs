//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1029/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1029<F: Float>(t26781: F, t72: F, t1927: F, t6977: F, t7575: F, t2122: F, t25146: F, t10309: F, t7565: F, t25163: F, t1923: F, t2123: F, t25102: F, t25110: F, t25114: F, t25117: F, t25120: F, t25150: F, t25159: F, t25162: F, t26749: F, t26755: F, t6954: F, t6960: F, t6963: F, t7566: F, t7576: F, t7579: F) -> (F, F, F, F, F, F, F) {
    let t26782 = t26781 * t72;
    let t26783 = t26782 * t1927;
    let t26786 = t7575 * t6977;
    let t26789 = t2122 * t25146;
    let t26792 = t10309 * t7565;
    let t26795 = t2122 * t25163;
    let t26798 = 5.0 / 3.0 * t26749 * t6960 + 2.0 / 3.0 * t25102 * t2123 + 5.0 / 3.0 * t26755 * t6960 + 5.0 / 3.0 * t7566 * t25110 + 5.0 / 6.0 * t7566 * t25114 + t25117 * t2123 / 3.0 + t25120 * t2123 / 3.0 + 2.0 / 3.0 * t6963 * t7576 + 2.0 / 3.0 * t6963 * t7579 - t25150 * t2123 / 6.0 - t6954 * t7576 / 3.0 - t6954 * t7579 / 3.0 - t1923 * t26783 / 6.0 - t1923 * t26786 / 3.0 - t1923 * t26789 / 6.0 - 5.0 * t26792 * t25159 - 10.0 / 3.0 * t25162 * t26795;
    (t26782, t26783, t26786, t26789, t26792, t26795, t26798)
}
