//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 931/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk931<F: Float>(t120361: F, t7150: F, t127: F, t32003: F, t32004: F, t371: F, t32010: F, t3215: F, t31950: F, t31951: F, t31912: F, t32013: F, t11921: F, t247: F, t31886: F, t8502: F) -> (F, F, F, F, F, F) {
    let t120362 = t7150 * t120361;
    let t120368 = t32003 * t371 * t127 * t32004;
    let t120370 = t32010 * t3215;
    let t120374 = t31950 * t371 * t127 * t31951;
    let t120376 = t31912 * t32013;
    let t120385 = t8502 * t247 * t11921 * t31886;
    (t120362, t120368, t120370, t120374, t120376, t120385)
}
