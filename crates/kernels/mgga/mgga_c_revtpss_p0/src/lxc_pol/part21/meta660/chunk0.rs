//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2453/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2453<F: Float>(t11858: F, t16048: F, t11859: F, t11861: F, t11922: F, t11927: F, t11929: F, t1065: F, t215: F, t1063: F, t247: F, t906: F) -> (F, F, F, F, F) {
    let t42765 = t11858 * t16048;
    let t42769 = t11859 * t11922 * t11861;
    let t42772 = t11927 * t11922 * t11929;
    let t42778 = t215 * t1065;
    let t42781 = t1063 * t247 * t42778 * t906;
    (t42765, t42769, t42772, t42778, t42781)
}
