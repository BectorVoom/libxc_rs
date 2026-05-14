//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 592/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk592<F: Float>(t1232: F, t1771: F, t1228: F, t8282: F, t2347: F, t852: F, t2360: F, t1212: F, t2781: F, t1240: F, t2842: F, t1250: F, t8232: F, t2770: F, t848: F, t319: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t15011 = t1771 * t1232;
    let t15025 = t8282 * t1228;
    let t15042 = t852 * t2347;
    let t15047 = t852 * t2360;
    let t15051 = t2781 * t1212;
    let t15128 = t1240 * t2842;
    let t15147 = t8232 * t1250;
    let t15191 = t2770 * t1240;
    let t15195 = t848 * t1240;
    let t15229 = t2770 * t319;
    (t15011, t15025, t15042, t15047, t15051, t15128, t15147, t15191, t15195, t15229)
}
