//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1178/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1178<F: Float>(t3022: F, t4725: F, t11465: F, t1633: F, t3015: F, t981: F, t3026: F, t4719: F, t1695: F, t3075: F, t1079: F, t3215: F, t4858: F, t372: F, t4872: F, t4786: F, t4873: F) -> (F, F, F, F, F, F, F) {
    let t15571 = 0.23392894490538584828e1 * t3022 * t4725;
    let t15572 = t11465 * t1633;
    let t15573 = t15572 * t3015;
    let t15575 = 0.10389515463408878255e3 * t981 * t15573;
    let t15577 = 0.11696447245269292414e1 * t4719 * t3026;
    let t15578 = t1695 * t3075;
    let t15579 = t1079 * t15578;
    let t15583 = 0.28582678745379824648e-3 * t4858 * t3215;
    let t15584 = t372 * t4872;
    let t15585 = t4873 * t4786;
    (t15571, t15575, t15577, t15579, t15583, t15584, t15585)
}
