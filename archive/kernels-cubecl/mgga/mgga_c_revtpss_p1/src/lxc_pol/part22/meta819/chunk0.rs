//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2931/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2931<F: Float>(t136: F, t2457: F, t5774: F, t9674: F, t10175: F, t14079: F, t10073: F, t13731: F, t3915: F, t5721: F, t9288: F, t2439: F, t3895: F, t5775: F) -> (F, F, F, F, F) {
    let t47885 = t9674 * t5774 * t136 * t2457;
    let t47893 = t10175 * t14079;
    let t47899 = t10073 * t13731;
    let t47904 = t3915 * t5721 * t9288;
    let t47907 = t2439 * t3895 * t5775;
    (t47885, t47893, t47899, t47904, t47907)
}
