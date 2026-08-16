//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta480 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1469;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta480<F: Float>(t3451: F, t6481: F, t12555: F, t6534: F, t3565: F, t6563: F, t225: F, t1261: F, t12879: F, t247: F, t6429: F, t11262: F, t1247: F, t6624: F) -> (F, F, F, F, F, F) {
        let (t69488, t69511, t69636, t69637, t69661, t69668) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1469::<F>(t3451, t6481, t12555, t6534, t3565, t6563, t225, t1261, t12879, t247, t6429, t11262, t1247, t6624);
    (t69488, t69511, t69636, t69637, t69661, t69668)
}
