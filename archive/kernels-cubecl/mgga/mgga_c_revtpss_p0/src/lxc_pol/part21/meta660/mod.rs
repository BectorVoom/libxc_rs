//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta660 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2453;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2454;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta660<F: Float>(t11858: F, t16048: F, t11859: F, t11861: F, t11922: F, t11927: F, t11929: F, t1065: F, t215: F, t1063: F, t247: F, t906: F, t11986: F, t2858: F, t11744: F, t3106: F, t373: F, t675: F, t828: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t42765, t42769, t42772, t42778, t42781) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2453::<F>(t11858, t16048, t11859, t11861, t11922, t11927, t11929, t1065, t215, t1063, t247, t906);
        let (t42785, t42788, t42792, t42793) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2454::<F>(t1063, t11986, t247, t2858, t11744, t3106, t373, t675, t828);
    (t42765, t42769, t42772, t42778, t42781, t42785, t42788, t42792, t42793)
}
