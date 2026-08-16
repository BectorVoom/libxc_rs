//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1152/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1152(t2122: f64, t25146: f64, t10309: f64, t7565: f64, t25163: f64, t1923: f64, t2123: f64, t25102: f64, t25110: f64, t25114: f64, t25117: f64, t25120: f64, t25150: f64, t25159: f64, t25162: f64, t26749: f64, t26755: f64, t26783: f64, t26786: f64, t6954: f64, t6960: f64, t6963: f64, t7566: f64, t7576: f64, t7579: f64) -> (f64, f64, f64, f64) {
    let t26789 = t2122 * t25146;
    let t26792 = t10309 * t7565;
    let t26795 = t2122 * t25163;
    let t26798 = 5.0_f64 / 3.0_f64 * t26749 * t6960 + 2.0_f64 / 3.0_f64 * t25102 * t2123 + 5.0_f64 / 3.0_f64 * t26755 * t6960 + 5.0_f64 / 3.0_f64 * t7566 * t25110 + 5.0_f64 / 6.0_f64 * t7566 * t25114 + t25117 * t2123 / 3.0_f64 + t25120 * t2123 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t6963 * t7576 + 2.0_f64 / 3.0_f64 * t6963 * t7579 - t25150 * t2123 / 6.0_f64 - t6954 * t7576 / 3.0_f64 - t6954 * t7579 / 3.0_f64 - t1923 * t26783 / 6.0_f64 - t1923 * t26786 / 3.0_f64 - t1923 * t26789 / 6.0_f64 - 5.0_f64 * t26792 * t25159 - 10.0_f64 / 3.0_f64 * t25162 * t26795;
    (t26789, t26792, t26795, t26798)
}
