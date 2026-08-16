//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1333/1444 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1333<F: Float>(t2212: F, t3738: F, t6791: F, t11626: F, t6179: F, t824: F, t11210: F, t11657: F, t15650: F, t10287: F, t190: F, t7108: F, t959: F) -> (F, F, F, F) {
    let t35973 = t3738 * t6791 * t2212;
    let t35976 = t824 * t6179 * t11626;
    let t35979 = t11657 * t11210 * t15650;
    let t35983 = t10287 * t190 * t959 * t7108;
    (t35973, t35976, t35979, t35983)
}
