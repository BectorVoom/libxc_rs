//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta355 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1665;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1666;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta355<F: Float>(t2710: F, t2713: F, t4371: F, t4353: F, t808: F, t10744: F, t10905: F, t4442: F, t240: F, t849: F, t14648: F, t775: F, t2661: F, t2652: F, t4345: F, t10716: F, t4349: F, t2689: F, t4372: F, t4354: F, t9775: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t14817, t14819, t14820, t14823, t14832) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1665::<F>(t2710, t2713, t4371, t4353, t808, t10744, t10905, t4442, t240, t849);
        let (t14833, t14834, t14836, t14837, t14839, t14846, t14850) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1666::<F>(t14648, t775, t14832, t2661, t2652, t4345, t10716, t4349, t2689, t4372, t4354, t9775);
    (t14817, t14819, t14820, t14823, t14832, t14833, t14834, t14836, t14837, t14839, t14846, t14850)
}
