//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1178/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1178<F: Float>(t32195: F, t32206: F, t5627: F, t9955: F, t125587: F, t32211: F, t3936: F, t13975: F, t246: F, t32289: F, t8591: F, t121126: F, t5673: F, t5727: F) -> (F, F, F, F) {
    let t125803 = t32206 * t9955 * t32195 * t5627;
    let t125807 = t32206 * t3936 * t32211 * t125587;
    let t125814 = t8591 * t32289 * t246 * t13975;
    let t125819 = t32206 * t5673 * t121126 * t5727;
    (t125803, t125807, t125814, t125819)
}
