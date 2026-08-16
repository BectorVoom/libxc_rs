//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2303/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2303(t1250: f64, t24739: f64, t3720: f64, t17661: f64, t6639: f64, t1794: f64, t6587: f64) -> (f64, f64, f64, f64) {
    let t24740 = t24739 * t1250;
    let t24741 = t3720 * t24740;
    let t24744 = t17661 * t6639;
    let t24751 = t6587 * t1794;
    (t24740, t24741, t24744, t24751)
}
