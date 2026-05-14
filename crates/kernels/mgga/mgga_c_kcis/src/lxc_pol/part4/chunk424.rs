//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 424/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk424<F: Float>(t1664: F, t921: F, t261: F, t920: F) -> (F, F, F) {
    let t1666 = -t921 - 0.17808333333333333333e-1 * t1664;
    let t1668 = 0.62182e-1 * t1666 * t261;
    let t1670 = -t920 / 3.0 - t1664 / 3.0;
    (t1666, t1668, t1670)
}
