//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1242/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1242(t102137: f64, t102139: f64, t102142: f64, t102173: f64, t102267: f64, t106836: f64, t106862: f64, t2032: f64, t26911: f64, t27966: f64, t27972: f64, t28935: f64, t7432: f64, t7435: f64, t7782: f64, t84216: f64, t91905: f64, t91922: f64) -> f64 {
    let t108727 = -2.0_f64 * t7435 * t28935 - 70.0_f64 * t84216 * t106836 - 5.0_f64 * t102267 * t7432 - 2.0_f64 * t106862 * t2032 - 10.0_f64 * t26911 * t27972 - 4.0_f64 * t27966 * t7782 - 16.0_f64 / 3.0_f64 * t102137 + 16.0_f64 / 3.0_f64 * t102139 - 8.0_f64 / 3.0_f64 * t102142 - 176.0_f64 / 9.0_f64 * t91905 - 440.0_f64 / 9.0_f64 * t91922 - 160.0_f64 / 3.0_f64 * t102173;
    t108727
}
