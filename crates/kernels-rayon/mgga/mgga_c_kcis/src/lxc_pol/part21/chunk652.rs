//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 652/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk652(t5180: f64, t5182: f64, t1175: f64, t1804: f64, t375: f64, t5084: f64, t5087: f64, t5089: f64, t5092: f64, t5094: f64, t5097: f64, t5100: f64, t5166: f64, t5170: f64, t5173: f64, t5178: f64) -> (f64, f64, f64, f64) {
    let t5183 = t5180 * t5182;
    let t5185 = t1175 * t1804;
    let t5186 = t375 * t5185;
    let t5188 = -t5084 / 16.0_f64 + t5087 / 36.0_f64 + t5089 / 256.0_f64 - t5092 / 24.0_f64 + t5094 / 48.0_f64 - t5097 / 48.0_f64 + t5100 / 192.0_f64 + t5166 / 16.0_f64 - t5170 / 256.0_f64 + t5173 / 256.0_f64 - t5178 / 128.0_f64 - t5183 / 576.0_f64 - t5186 / 9.0_f64;
    (t5183, t5185, t5186, t5188)
}
