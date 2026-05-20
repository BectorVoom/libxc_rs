//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2520/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2520<F: Float>(t1214: F, t17703: F, t1243: F, t42859: F, t460: F, t1204: F, t13126: F, t12722: F, t3566: F, t5462: F, t5477: F, t1209: F, t1284: F, t3727: F) -> (F, F, F, F, F, F, F, F) {
    let t45796 = t17703 * t1214;
    let t45832 = t42859 * t1243;
    let t45833 = t460 * t45832;
    let t45846 = t1204 * t13126;
    let t45852 = t3566 * t12722;
    let t45859 = t3566 * t5462;
    let t45863 = t3566 * t5477;
    let t45868 = t1209 * t1284 * t3727;
    (t45796, t45832, t45833, t45846, t45852, t45859, t45863, t45868)
}
