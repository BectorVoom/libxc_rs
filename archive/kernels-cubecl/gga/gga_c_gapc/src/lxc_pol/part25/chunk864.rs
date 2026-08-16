//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 864/1444 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk864<F: Float>(t7420: F, t9838: F, t291: F, t8785: F, t1734: F, t1084: F, t2546: F, t3328: F, t4: F, t5: F) -> (F, F, F, F) {
    let t9839 = t9838 * t7420;
    let t9841 = t8785 * t291;
    let t9842 = t1734 * t9841;
    let t9843 = t1084 * t9842;
    let t9846 = t2546 * t5 * t3328 * t4;
    (t9839, t9841, t9843, t9846)
}
