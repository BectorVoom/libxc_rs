//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta656 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2445;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2446;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta656<F: Float>(t11804: F, t11921: F, t247: F, t4837: F, t1063: F, t11169: F, t3109: F, t1011: F, t11758: F, t140: F, t11823: F, t11828: F, t11144: F, t3252: F, t11852: F, t126: F, t11145: F, t11679: F, t11710: F, t3091: F, t11247: F, t11249: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t42487, t42496, t42499, t42506, t42516) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2445::<F>(t11804, t11921, t247, t4837, t1063, t11169, t3109, t1011, t11758, t140, t11823, t11828);
        let (t42518, t42537, t42546, t42550) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2446::<F>(t11144, t3252, t11852, t126, t1063, t11145, t247, t11679, t11710, t3091, t11247, t11249);
    (t42487, t42496, t42499, t42506, t42516, t42518, t42537, t42546, t42550)
}
