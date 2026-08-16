//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1447/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1447<F: Float>(t10744: F, t18409: F, t808: F, t18414: F, t40521: F, t40791: F, t5989: F, t10890: F, t5985: F, t10760: F, t40627: F, t61837: F) -> (F, F, F, F, F) {
    let t62069 = t10744 * t808 * t18409;
    let t62072 = t40521 * t808 * t18414;
    let t62089 = t40791 * t5989;
    let t62095 = t10890 * t5985;
    let t62111 = t10760 * t40627 * t61837;
    (t62069, t62072, t62089, t62095, t62111)
}
