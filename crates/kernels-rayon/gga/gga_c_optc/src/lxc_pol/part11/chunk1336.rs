//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1336/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1336(t23769: f64, t23770: f64, t30189: f64, t30270: f64, t49378: f64, t49381: f64, t56978: f64, t56981: f64, t56984: f64, t56988: f64, t56991: f64, t56994: f64) -> f64 {
    let t58143 = -26169.0_f64 * t56978 + 0.58153333333333333332e4_f64 * t56981 - 0.19384444444444444444e4_f64 * t56984 - 2832.0_f64 * t56988 + 0.62933333333333333332e3_f64 * t56991 + 0.94399999999999999998e3_f64 * t56994 + 0.93234567901234567903e3_f64 * t30189 + t23769 + t23770 + 0.932345679012345679e2_f64 * t49378 + 0.20977777777777777778e3_f64 * t49381 + 0.30153580246913580247e4_f64 * t30270;
    t58143
}
