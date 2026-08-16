//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1208/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1208<F: Float>(t1211: F, t12646: F, t1214: F, t3790: F, t1277: F, t3552: F, t487: F, t1208: F, t3551: F) -> (F, F, F, F) {
    let t12647 = t1211 * t12646;
    let t12650 = t1214 * t3790;
    let t12651 = t1277 * t12650;
    let t12654 = t3552 * t487;
    let t12657 = t3551 * t1208;
    (t12647, t12651, t12654, t12657)
}
