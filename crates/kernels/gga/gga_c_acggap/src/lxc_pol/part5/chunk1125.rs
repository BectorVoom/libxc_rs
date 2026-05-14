//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1125/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1125<F: Float>(t5821: F, t997: F, t5811: F, t5546: F, t14056: F, t6140: F, t3391: F, t4680: F, t6143: F, t1181: F, t1432: F, t15995: F, t1106: F, t1899: F, t1165: F, t17710: F, t17718: F, t17721: F, t17725: F, t3396: F, t4665: F, t6138: F) -> (F,) {
    let t23063 = t997 * t5821;
    let t23065 = t997 * t5811;
    let t23068 = t997 * t5546;
    let t23070 = t14056 * t6140;
    let t23077 = t3391 * t4680 * t6143;
    let t23081 = t3391 * t1181 * t15995 * t1432;
    let t23088 = t3391 * t1181 * t1899 * t1106;
    let t23090 = 0.16006300097412701803e-1 * t23063 + 0.16006300097412701803e-1 * t23065 - 0.68598428988911579156e-2 * t17710 - 0.32012600194825403606e-1 * t23068 - 0.20579528696673473746e-1 * t23070 - 0.10289764348336736873e-1 * t3396 * t1165 * t6138 * t4665 + 0.34299214494455789578e-2 * t23077 + 0.34299214494455789578e-2 * t23081 + 0.68598428988911579156e-2 * t17718 + 0.34299214494455789578e-2 * t17721 + 0.34299214494455789578e-2 * t17725 + 0.17149607247227894789e-2 * t23088;
    (t23090,)
}
