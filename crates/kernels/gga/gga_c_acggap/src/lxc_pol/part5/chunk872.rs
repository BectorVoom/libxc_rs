//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 872/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk872<F: Float>(t1381: F, t2955: F, t224: F, t4064: F, t229: F, t2974: F, t484: F, t2977: F, t5042: F, t691: F, t276: F, t40: F, t4027: F, t1284: F, t228: F, t1292: F) -> (F, F, F, F, F, F, F, F, F) {
    let t15005 = t1381 * t2955;
    let t15008 = t224 * t4064;
    let t15010 = t229 * t4064;
    let t15016 = t2974 * t484;
    let t15018 = t2977 * t484;
    let t15043 = t5042 * t691;
    let t15050 = t40 * t4027 * t276;
    let t15072 = 16.0 * t1284 * t228;
    let t15095 = 16.0 * t1292 * t228;
    (t15005, t15008, t15010, t15016, t15018, t15043, t15050, t15072, t15095)
}
