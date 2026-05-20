//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta473 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2173;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta473<F: Float>(t3172: F, t4874: F, t3127: F, t4802: F, t1063: F, t4807: F, t11723: F, t11728: F, t11730: F, t11732: F, t11737: F, t11745: F, t15758: F, t3106: F, t4803: F, t4808: F, t4896: F) -> (F, F, F, F, F, F, F) {
        let (t15769, t15771, t15772, t15774, t15775, t15776, t15779) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2173::<F>(t3172, t4874, t3127, t4802, t1063, t4807, t11723, t11728, t11730, t11732, t11737, t11745, t15758, t3106, t4803, t4808, t4896);
    (t15769, t15771, t15772, t15774, t15775, t15776, t15779)
}
