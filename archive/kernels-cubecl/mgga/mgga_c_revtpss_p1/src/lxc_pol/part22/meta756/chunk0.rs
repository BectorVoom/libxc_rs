//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2833/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2833<F: Float>(t1063: F, t11986: F, t247: F, t2862: F, t11880: F, t3241: F, t1011: F, t1016: F, t2438: F, t3237: F, t697: F, t1014: F, t11150: F) -> (F, F, F, F, F) {
    let t42710 = t1063 * t247 * t11986 * t2862;
    let t42712 = t3241 * t11880;
    let t42716 = t1011 * t2438 * t1016;
    let t42719 = t1011 * t697 * t3237;
    let t42731 = t1014 * t11150;
    (t42710, t42712, t42716, t42719, t42731)
}
