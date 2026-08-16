//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta731 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2576;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2577;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta731<F: Float>(t675: F, t9898: F, t268: F, t4101: F, t543: F, t14192: F, t555: F, t786: F, t9994: F, t10023: F, t4003: F, t10115: F, t1441: F, t10008: F, t545: F, t689: F, t869: F, t4093: F, t9292: F, t10065: F, t10073: F, t1432: F, t1433: F, t39497: F, t1385: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t47369, t47371, t47375, t47379, t47381) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2576::<F>(t675, t9898, t268, t4101, t543, t14192, t555, t786, t9994, t10023, t4003, t10115, t1441);
        let (t47387, t47389, t47391, t47395, t47396) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2577::<F>(t10008, t545, t689, t869, t4093, t9292, t10065, t10073, t1432, t1433, t39497, t1385);
    (t47369, t47371, t47375, t47379, t47381, t47387, t47389, t47391, t47395, t47396)
}
