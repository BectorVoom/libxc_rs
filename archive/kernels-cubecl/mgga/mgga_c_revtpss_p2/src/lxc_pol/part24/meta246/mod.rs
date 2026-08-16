//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta246 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1009;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta246<F: Float>(t10769: F, t828: F, t2746: F, t2710: F, t2713: F, t4371: F, t4353: F, t808: F, t10744: F, t240: F, t849: F, t10716: F, t4349: F) -> (F, F, F, F, F, F, F) {
        let (t14785, t14791, t14817, t14819, t14820, t14832, t14839) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1009::<F>(t10769, t828, t2746, t2710, t2713, t4371, t4353, t808, t10744, t240, t849, t10716, t4349);
    (t14785, t14791, t14817, t14819, t14820, t14832, t14839)
}
