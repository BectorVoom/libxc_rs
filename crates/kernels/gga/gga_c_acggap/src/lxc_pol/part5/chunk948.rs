//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 948/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk948<F: Float>(t288: F, t4027: F, t75: F, t5042: F, t682: F, t1381: F, t2955: F, t224: F, t4064: F, t229: F, t2974: F, t484: F) -> (F, F, F, F, F, F) {
    let t14999 = t4027 * t75 * t288;
    let t15003 = t5042 * t682;
    let t15005 = t1381 * t2955;
    let t15008 = t224 * t4064;
    let t15010 = t229 * t4064;
    let t15016 = t2974 * t484;
    (t14999, t15003, t15005, t15008, t15010, t15016)
}
