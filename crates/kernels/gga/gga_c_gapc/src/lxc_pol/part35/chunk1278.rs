//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 1278/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk1278<F: Float>(t10123: F, t10243: F, t2531: F, t329: F, t827: F, t6210: F, t959: F, t11687: F, t6951: F, t11682: F, t6943: F, t11683: F, t23579: F) -> (F, F, F, F) {
    let t35788 = t10243 * t827 * t10123 * t329 * t2531;
    let t35790 = t6210 * t959;
    let t35792 = t11687 * t35790 * t6951;
    let t35795 = t11682 * t35790 * t6943;
    let t35798 = t11682 * t11683 * t23579;
    (t35788, t35792, t35795, t35798)
}
