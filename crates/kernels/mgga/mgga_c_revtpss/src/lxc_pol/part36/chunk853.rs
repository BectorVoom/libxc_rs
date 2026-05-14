//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 853/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk853<F: Float>(t4820: F, t4879: F, t11725: F, t247: F, t6092: F, t1063: F, t3109: F, t6100: F, t1647: F, t1678: F, t378: F, t6235: F, t4746: F, t6343: F, t994: F, t19462: F) -> (F, F, F, F, F, F, F, F) {
    let t20034 = t4879 * t4820;
    let t20050 = t247 * t11725 * t6092;
    let t20051 = t1063 * t20050;
    let t20054 = t247 * t3109 * t6100;
    let t20055 = t1063 * t20054;
    let t20175 = t1647 * t1678;
    let t20178 = t6235 * t378;
    let t20191 = t4746 * t1678;
    let t20204 = t994 * t6343;
    let t20211 = t19462 * t378;
    (t20034, t20051, t20055, t20175, t20178, t20191, t20204, t20211)
}
