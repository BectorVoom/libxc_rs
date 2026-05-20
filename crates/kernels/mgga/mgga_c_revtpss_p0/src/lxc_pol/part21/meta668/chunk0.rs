//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2468/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2468<F: Float>(t1086: F, t11200: F, t3090: F, t11671: F, t11926: F, t1045: F, t2862: F, t999: F, t3075: F, t606: F, t16565: F, t994: F) -> (F, F, F, F, F) {
    let t43291 = t11200 * t1086 * t3090;
    let t43297 = t11926 * t11671;
    let t43301 = t1045 * t2862 * t999;
    let t43313 = t606 * t3075;
    let t43341 = t994 * t16565;
    (t43291, t43297, t43301, t43313, t43341)
}
