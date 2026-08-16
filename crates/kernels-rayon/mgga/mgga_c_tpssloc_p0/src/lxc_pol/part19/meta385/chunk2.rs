//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1444/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1444(t43776: f64, t43837: f64, t43839: f64, t43842: f64, t43845: f64, t43848: f64, t43851: f64, t43855: f64, t43857: f64, t43859: f64, t43861: f64, t43863: f64) -> f64 {
    let t44466 = 220.0_f64 / 81.0_f64 * t43776;
    let t44470 = 8.0_f64 / 3.0_f64 * t43837 + 4.0_f64 / 9.0_f64 * t43839 - 8.0_f64 / 9.0_f64 * t43842 + 2.0_f64 * t43845 - 4.0_f64 * t43848 - t43851 / 6.0_f64 + 10.0_f64 / 27.0_f64 * t43855 + 16.0_f64 / 81.0_f64 * t43857 - t44466 + 160.0_f64 / 81.0_f64 * t43859 - 10.0_f64 / 9.0_f64 * t43861 - 20.0_f64 / 9.0_f64 * t43863;
    t44470
}
