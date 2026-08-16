//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 256/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk256(t1061: f64, t721: f64, t1060: f64, t130: f64, t39: f64, t14: f64, t25: f64) -> (f64, f64, f64, f64) {
    let t1062 = t1061 * t721;
    let t1063 = t1060 * t1062;
    let t1068 = t130 * t39;
    let t1072 = 1.0_f64 / t14 / t25 / 4.0_f64;
    (t1062, t1063, t1068, t1072)
}
