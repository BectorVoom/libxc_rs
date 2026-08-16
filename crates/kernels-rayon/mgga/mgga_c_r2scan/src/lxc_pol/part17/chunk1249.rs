//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1249/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1249(t1044: f64, t1149: f64, t12365: f64, t12964: f64, t44147: f64, t44150: f64, t44152: f64, t44155: f64, t44158: f64, t44161: f64, t44165: f64, t44168: f64, t44519: f64, t44524: f64, t44526: f64, t44530: f64, t44532: f64, t44535: f64, t860: f64, t9782: f64) -> f64 {
    let t44536 = 2.0_f64 * t1044 * t12365 + t1149 * t9782 + t12964 * t860 + t44147 - t44150 + t44152 + t44155 + t44158 - t44161 + t44165 + t44168 + t44519 - t44524 - t44526 + t44530 - t44532 - t44535;
    t44536
}
