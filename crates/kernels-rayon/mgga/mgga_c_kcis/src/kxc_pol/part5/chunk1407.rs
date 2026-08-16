//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1407/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1407(t1599: f64, t1612: f64, t18223: f64, t23158: f64, t23174: f64, t23178: f64, t23182: f64, t23186: f64, t23192: f64, t23194: f64, t23200: f64, t23208: f64, t23211: f64, t23213: f64, t6141: f64, t6179: f64, t6185: f64) -> f64 {
    let t23215 = t23174 / 1296.0_f64 - t1599 * t23178 / 32.0_f64 + t1599 * t23182 / 48.0_f64 + t1599 * t23186 / 576.0_f64 - t6141 * t6179 / 18.0_f64 - t23192 / 864.0_f64 - t23194 / 324.0_f64 - t18223 / 432.0_f64 - t1599 * t23200 / 192.0_f64 - 11.0_f64 / 216.0_f64 * t23158 * t1612 + t6141 * t6185 / 36.0_f64 - t23208 / 576.0_f64 + t23211 / 288.0_f64 + t23213 / 108.0_f64;
    t23215
}
