//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta128 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk685;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk686;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk687;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta128(t3507: f64, t475: f64, t1214: f64, t248: f64, t121: f64, t1229: f64, t1090: f64, t1227: f64, t1230: f64, t3252: f64, t3248: f64, t1009: f64, t1190: f64, t1011: f64, t1212: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t3516 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk685(t3507, t475);
        let (t3518, t3521) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk686(t1214, t248, t3516, t121, t1229);
        let (t3523, t3524, t3527, t3531, t3534, t3536) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk687(t1090, t248, t3521, t1227, t1230, t3252, t3248, t1009, t1190, t1011, t1212);
    (t3516, t3518, t3521, t3523, t3524, t3527, t3531, t3534, t3536)
}
