//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 20/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk20(t12: f64, t15: f64, t18: f64, t26: f64, t14: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64) {
    let t28 = 0.379785e1_f64 * t15 + 0.8969e0_f64 * t12 + 0.204775e0_f64 * t18 + 0.123235e0_f64 * t26;
    let t31 = 1.0_f64 + 0.16081824322151104822e2_f64 / t28;
    let t32 = f64::ln(t31);
    let t34 = 0.62182e-1_f64 * t14 * t32;
    let t36 = pow_1_3(zeta_threshold);
    let t37 = t36 * zeta_threshold;
    (t28, t31, t32, t34, t37)
}
