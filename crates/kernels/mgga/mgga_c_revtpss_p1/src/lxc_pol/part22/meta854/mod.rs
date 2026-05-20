//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta854 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2997;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2998;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta854<F: Float>(t10115: F, t1570: F, t11007: F, t1579: F, t252: F, t2771: F, t2782: F, t4322: F, t9292: F, t2772: F, t4321: F, t689: F, t11024: F, t1580: F, t10981: F, t22: F, t868: F, t15060: F, t2435: F, t14982: F, t2465: F, t2470: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t50155, t50161, t50164, t50166, t50169) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2997::<F>(t10115, t1570, t11007, t1579, t252, t2771, t2782, t4322, t9292, t2772, t4321, t689);
        let (t50174, t50178, t50183, t50186) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2998::<F>(t11024, t1580, t689, t10981, t1579, t22, t868, t15060, t2435, t14982, t2465, t2470);
    (t50155, t50161, t50164, t50166, t50169, t50174, t50178, t50183, t50186)
}
