//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1944/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1944<F: Float>(t225: F, t29807: F, t385: F, t1982: F, t6343: F, t1695: F, t7821: F, t7160: F, t1089: F, t1668: F, t27604: F, t6299: F, t7168: F) -> (F, F, F, F, F, F) {
    let t29809 = t29807 * t225 * t385;
    let t29812 = t1982 * t6343;
    let t29817 = t7821 * t1695;
    let t29818 = t7160 * t29817;
    let t29822 = t27604 * t1668 * t1089;
    let t29826 = t7168 * t6299 * t1089;
    (t29809, t29812, t29817, t29818, t29822, t29826)
}
