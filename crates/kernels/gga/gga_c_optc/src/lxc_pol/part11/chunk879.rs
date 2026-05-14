//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 879/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk879<F: Float>(t17383: F, t8611: F, t11671: F, t11677: F, t14881: F, t14883: F, t14885: F, t14887: F, t14889: F, t14895: F, t17381: F, t17384: F, t17389: F, t17392: F, t8640: F, t4095: F, t5133: F) -> (F, F, F) {
    let t17394 = t8611 * t17383;
    let t17396 = -0.32862666666666666666e0 * t14881 + 0.16431333333333333333e0 * t14883 + 0.19931111111111111111e0 * t14885 - 0.59793333333333333333e0 * t14887 + 0.29896666666666666667e0 * t14889 + 0.5477111111111111111e-1 * t14895 + 0.1898925e1 * t17381 + 0.142419375e1 * t17384 - 0.39862222222222222223e0 * t11671 - 0.27385555555555555556e0 * t11677 - 0.82156666666666666668e-1 * t17389 + 0.49293999999999999999e0 * t17392 - 0.76790625e-1 * t17394 - t8640;
    let t17399 = t4095 * t5133;
    (t17394, t17396, t17399)
}
