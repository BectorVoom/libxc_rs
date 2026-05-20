//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta677 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2414;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2415;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta677<F: Float>(t13041: F, t44173: F, t13061: F, t13100: F, t828: F, t12879: F, t1247: F, t1251: F, t42994: F, t1231: F, t12898: F, t43813: F, t12256: F, t3698: F, t3362: F, t414: F, t12884: F, t3555: F, t3766: F, t5330: F, t1209: F, t13147: F, t17708: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t44174, t44202, t44225, t44250, t44264, t44291, t44307) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2414::<F>(t13041, t44173, t13061, t13100, t828, t12879, t1247, t1251, t42994, t1231, t12898, t43813);
        let (t44348, t44361, t44425, t44484, t44500) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2415::<F>(t12256, t3698, t3362, t414, t12884, t828, t3555, t3766, t5330, t1209, t13147, t17708);
    (t44174, t44202, t44225, t44250, t44264, t44291, t44307, t44348, t44361, t44425, t44484, t44500)
}
