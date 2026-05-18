//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 809/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk809<F: Float>(t25877: F, t25894: F, t25881: F, t1385: F, t2028: F, t25875: F, t1399: F, t676: F, t25880: F, t212: F, t7274: F, t1358: F) -> (F, F, F, F, F, F, F, F, F) {
    let t25895 = t25894 * t25877;
    let t25896 = t25895 * t25881;
    let t25898 = t2028 * t1385;
    let t25899 = t25875 * t25898;
    let t25900 = t676 * t1399;
    let t25901 = t25880 * t25900;
    let t25902 = t25899 * t25901;
    let t25904 = t25894 * t25898;
    let t25905 = t25904 * t25901;
    let t25912 = t212 * t7274;
    let t25913 = t25912 * t1358;
    (t25895, t25896, t25898, t25899, t25900, t25902, t25904, t25905, t25913)
}
