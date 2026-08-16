//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta432 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2059;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta432<F: Float>(t10815: F, t1561: F, t2741: F, t4426: F, t10845: F, t4430: F, t1558: F, t853: F, t2749: F, t2662: F, t2661: F, t4352: F, t837: F) -> (F, F, F, F, F, F, F) {
        let (t14712, t14715, t14716, t14718, t14720, t14722, t14723) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2059::<F>(t10815, t1561, t2741, t4426, t10845, t4430, t1558, t853, t2749, t2662, t2661, t4352, t837);
    (t14712, t14715, t14716, t14718, t14720, t14722, t14723)
}
