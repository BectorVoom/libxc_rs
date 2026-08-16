//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 847/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk847(t1764: f64, t7148: f64, t7051: f64, t5218: f64, t572: f64, t2735: f64, t185: f64, t1019: f64, t1680: f64, t1791: f64, t2722: f64, t661: f64) -> (f64, f64, f64, f64) {
    let t7149 = t7148 * t1764;
    let t7150 = t7149 * t7051;
    let t7152 = 32.0_f64 / 45.0_f64 * t5218 * t7150;
    let t7153 = t7148 * t572;
    let t7154 = t2735 * t7153;
    let t7156 = 8.0_f64 / 45.0_f64 * t185 * t7154;
    let t7158 = 4.0_f64 / 15.0_f64 * t1680 * t1019;
    let t7159 = t1791 * t2722;
    let t7160 = t7159 * t661;
    (t7152, t7156, t7158, t7160)
}
