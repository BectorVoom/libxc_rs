//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 1296/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk1296<F: Float>(t11210: F, t11657: F, t15650: F, t10287: F, t190: F, t7108: F, t959: F, t10153: F, t3727: F, t6182: F, t3243: F, t6188: F) -> (F, F, F, F) {
    let t35979 = t11657 * t11210 * t15650;
    let t35983 = t10287 * t190 * t959 * t7108;
    let t35986 = t10153 * t3727 * t6182;
    let t35989 = t3243 * t3727 * t6188;
    (t35979, t35983, t35986, t35989)
}
