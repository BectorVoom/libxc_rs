//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 316/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk316(t193: f64, t202: f64, t680: f64, t705: f64, t710: f64, t719: f64, t752: f64, t755: f64, t760: f64, t765: f64, t766: f64, t776: f64, t868: f64, t870: f64) -> f64 {
    let t873 = t193 * t202 * t868 * t870 + 3.0_f64 * t193 * t766 * t776 + t680 + t705 + t710 + t719 + t752 + t755 - t760 - t765;
    t873
}
