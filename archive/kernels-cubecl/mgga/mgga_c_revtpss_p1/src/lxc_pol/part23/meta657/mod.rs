//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta657 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2386;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2387;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta657<F: Float>(t11003: F, t9303: F, t10981: F, t22: F, t868: F, t886: F, t2445: F, t9292: F, t588: F, t780: F, t39497: F, t787: F, t788: F, t2448: F, t10994: F, t2453: F, t11043: F, t11029: F, t39501: F, t781: F, t252: F, t257: F, t268: F, t39644: F, t8779: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t40970, t40978, t40988, t40998, t41003) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2386::<F>(t11003, t9303, t10981, t22, t868, t886, t2445, t9292, t588, t780, t39497, t787, t788);
        let (t41004, t41011, t41020, t41034, t41037, t41049) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2387::<F>(t2448, t9292, t10994, t2453, t11043, t11029, t9303, t39501, t781, t252, t257, t268, t39644, t8779);
    (t40970, t40978, t40988, t40998, t41003, t41004, t41011, t41020, t41034, t41037, t41049)
}
