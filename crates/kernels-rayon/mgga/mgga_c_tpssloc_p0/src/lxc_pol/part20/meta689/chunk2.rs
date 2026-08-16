//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2613/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2613(t1227: f64, t13969: f64, t15616: f64, t11731: f64, t11741: f64, t11781: f64, t45007: f64, t45009: f64, t45013: f64, t5024: f64, t53079: f64, t53083: f64, t53087: f64, t53093: f64, t53097: f64, t53099: f64) -> f64 {
    let t53102 = t1227 * t13969 * t15616;
    let t53106 = t53079 / 10368.0_f64 + t53083 * t11731 / 96.0_f64 - t53087 * t11741 / 576.0_f64 + 5.0_f64 / 972.0_f64 * t5024 * t11781 + t53093 / 256.0_f64 + t53097 + t45007 / 4608.0_f64 + t53099 / 10368.0_f64 - t53102 / 384.0_f64 - t45009 / 2304.0_f64 - t45013 / 6912.0_f64;
    t53106
}
