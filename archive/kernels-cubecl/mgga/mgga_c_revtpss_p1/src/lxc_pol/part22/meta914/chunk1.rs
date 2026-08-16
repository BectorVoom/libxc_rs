//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3122/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3122<F: Float>(t15690: F, t3153: F, t372: F, t11921: F, t15716: F, t15717: F, t247: F, t1041: F, t1670: F, t42994: F, t11922: F, t15786: F, t4892: F) -> (F, F, F, F) {
    let t55209 = t372 * t15690 * t3153;
    let t55233 = t15716 * t247 * t11921 * t15717;
    let t55247 = t1041 * t42994 * t1670;
    let t55265 = t4892 * t11922 * t15786;
    (t55209, t55233, t55247, t55265)
}
