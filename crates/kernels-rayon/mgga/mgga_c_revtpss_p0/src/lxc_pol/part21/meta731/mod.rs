//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta731 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2576;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2577;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta731(t675: f64, t9898: f64, t268: f64, t4101: f64, t543: f64, t14192: f64, t555: f64, t786: f64, t9994: f64, t10023: f64, t4003: f64, t10115: f64, t1441: f64, t10008: f64, t545: f64, t689: f64, t869: f64, t4093: f64, t9292: f64, t10065: f64, t10073: f64, t1432: f64, t1433: f64, t39497: f64, t1385: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t47369, t47371, t47375, t47379, t47381) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2576(t675, t9898, t268, t4101, t543, t14192, t555, t786, t9994, t10023, t4003, t10115, t1441);
        let (t47387, t47389, t47391, t47395, t47396) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2577(t10008, t545, t689, t869, t4093, t9292, t10065, t10073, t1432, t1433, t39497, t1385);
    (t47369, t47371, t47375, t47379, t47381, t47387, t47389, t47391, t47395, t47396)
}
