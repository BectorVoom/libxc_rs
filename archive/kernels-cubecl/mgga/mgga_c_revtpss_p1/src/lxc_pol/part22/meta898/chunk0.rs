//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3090/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3090<F: Float>(t42865: F, t72: F, t3088: F, t43472: F, t43401: F, t11710: F, t15969: F, t4892: F, t1062: F, t15655: F, t11643: F, t15707: F) -> (F, F, F, F, F, F, F) {
    let t53667 = t42865 * t72;
    let t53668 = t3088 * t53667;
    let t53669 = t43472 * t53668;
    let t53676 = t43401 * t53668;
    let t53690 = t4892 * t11710 * t15969;
    let t53692 = t15655 * t1062;
    let t53710 = t15707 * t11643;
    (t53667, t53668, t53669, t53676, t53690, t53692, t53710)
}
