//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2935/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2935<F: Float>(t22: F, t46389: F, t543: F, t5735: F, t1432: F, t5763: F, t9288: F, t1892: F, t3923: F, t2782: F, t4003: F, t5744: F) -> (F, F, F, F) {
    let t47967 = t46389 * t5735 * t543 * t22;
    let t47971 = t1432 * t5763 * t9288;
    let t47973 = t1892 * t3923;
    let t47976 = t2782 * t5744 * t47973 * t4003;
    (t47967, t47971, t47973, t47976)
}
