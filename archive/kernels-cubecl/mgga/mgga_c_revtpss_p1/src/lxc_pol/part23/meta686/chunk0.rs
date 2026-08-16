//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2427/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2427<F: Float>(t1214: F, t45739: F, t1204: F, t13147: F, t13141: F, t3596: F, t42859: F, t460: F, t1243: F, t13126: F, t12722: F, t3566: F) -> (F, F, F, F, F, F, F, F, F) {
    let t45740 = t45739 * t1214;
    let t45769 = t1204 * t13147;
    let t45779 = t1204 * t13141;
    let t45785 = t42859 * t3596;
    let t45786 = t460 * t45785;
    let t45832 = t42859 * t1243;
    let t45833 = t460 * t45832;
    let t45846 = t1204 * t13126;
    let t45852 = t3566 * t12722;
    (t45740, t45769, t45779, t45785, t45786, t45832, t45833, t45846, t45852)
}
