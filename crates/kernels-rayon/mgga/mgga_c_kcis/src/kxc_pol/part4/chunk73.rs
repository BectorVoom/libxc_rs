//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 73/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk73(t12: f64, t15: f64, t18: f64, t26: f64, t187: f64, t189: f64, t34: f64, t57: f64) -> (f64, f64, f64, f64) {
    let t194 = 0.705945e1_f64 * t15 + 0.1549425e1_f64 * t12 + 0.420775e0_f64 * t18 + 0.1562925e0_f64 * t26;
    let t197 = 1.0_f64 + 0.32164683177870697974e2_f64 / t194;
    let t198 = f64::ln(t197);
    let t206 = -t34 + t187 * (-0.3109e-1_f64 * t189 * t198 + t34 - 0.19751789702565206229e-1_f64 * t57) + 0.19751789702565206229e-1_f64 * t187 * t57;
    (t194, t197, t198, t206)
}
