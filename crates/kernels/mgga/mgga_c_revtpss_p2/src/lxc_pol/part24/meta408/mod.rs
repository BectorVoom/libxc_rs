//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta408 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1349;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1350;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta408<F: Float>(t123: F, t212: F, t9291: F, t10981: F, t588: F, t780: F, t39497: F, t787: F, t788: F, t10994: F, t2453: F, t39501: F, t781: F, t252: F, t257: F, t268: F, t39644: F, t8779: F, t11007: F, t786: F, t11006: F, t256: F, t225: F, t2441: F, t39515: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t40921, t40998, t41003, t41011, t41037) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1349::<F>(t123, t212, t9291, t10981, t588, t780, t39497, t787, t788, t10994, t2453, t39501, t781);
        let (t41049, t41070, t41078, t41095) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1350::<F>(t252, t257, t268, t39644, t8779, t11007, t786, t11006, t256, t225, t2441, t39515);
    (t40921, t40998, t41003, t41011, t41037, t41049, t41070, t41078, t41095)
}
