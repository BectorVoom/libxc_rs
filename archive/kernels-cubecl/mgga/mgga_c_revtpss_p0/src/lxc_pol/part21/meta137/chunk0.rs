//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 882/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk882<F: Float>(t3230: F, t351: F, t1054: F, t1058: F, t1014: F, t2857: F, t2251: F, t1012: F, t1010: F, t614: F) -> (F, F, F, F, F) {
    let t3231 = t351 * t3230;
    let t3234 = t1054 * t1058;
    let t3236 = t1014 * t2857;
    let t3237 = t3236 * t2251;
    let t3238 = t1012 * t3237;
    let t3241 = t614 * t1010;
    (t3231, t3234, t3237, t3238, t3241)
}
