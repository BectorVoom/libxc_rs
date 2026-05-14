//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 772/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk772<F: Float>(t3728: F, t5678: F, t1494: F, t2001: F, t13396: F, t1392: F, t86: F, t5782: F, t2007: F, t3245: F, t1014: F, t5891: F, t5629: F, t14249: F, t5446: F, t3255: F, t5460: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t15941 = t3728 * t5678;
    let t15942 = 0.66327777777777777776e-2 * t15941;
    let t15955 = t1494 * t2001;
    let t15967 = t86 * t13396 * t1392;
    let t15968 = t15967 * t5782;
    let t15983 = t3245 * t2007;
    let t15986 = t1014 * t5891;
    let t15987 = 0.88437037037037037034e-2 * t15986;
    let t15988 = t3728 * t5629;
    let t15989 = 0.33163888888888888888e-2 * t15988;
    let t15994 = t14249 * t5446;
    let t16001 = 0.98556445e-3 * t3255 * t5460;
    (t15941, t15942, t15955, t15967, t15968, t15983, t15986, t15987, t15988, t15989, t15994, t16001)
}
