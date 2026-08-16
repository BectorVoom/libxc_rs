//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2583/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2583<F: Float>(t10009: F, t1364: F, t786: F, t3899: F, t4078: F, t689: F, t10162: F, t9303: F, t3903: F, t9292: F, t1445: F, t2439: F, t9640: F) -> (F, F, F, F, F) {
    let t47490 = t786 * t10009 * t1364;
    let t47493 = t689 * t3899 * t4078;
    let t47495 = t9303 * t10162;
    let t47497 = t9292 * t3903;
    let t47500 = t2439 * t9640 * t1445;
    (t47490, t47493, t47495, t47497, t47500)
}
