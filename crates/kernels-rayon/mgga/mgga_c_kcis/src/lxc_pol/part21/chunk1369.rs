//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1369/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1369(t1250: f64, t251: f64, t47323: f64, t27030: f64, t27070: f64, t28153: f64, t28190: f64, t7775: f64, t8087: f64, t92604: f64, t93056: f64, t96141: f64, t96154: f64, t96157: f64, t96160: f64, t96178: f64, t96181: f64, t96184: f64) -> f64 {
    let t97297 = t47323 * t251 * t1250;
    let t97303 = -0.11607361111111111111e-2_f64 * t96141 + 0.23214722222222222222e-2_f64 * t96154 + 0.61905925925925925926e-2_f64 * t96157 - 0.18534722222222222222e-2_f64 * t92604 * t8087 + 0.11607361111111111111e-2_f64 * t96160 + 0.46377350260416666667e-4_f64 * t93056 * t8087 + 0.92754700520833333334e-4_f64 * t27070 * t28153 - 0.69505208333333333334e-3_f64 * t28190 * t27030 + 0.92754700520833333334e-4_f64 * t97297 * t7775 + 0.11607361111111111111e-2_f64 * t96178 - 0.38691203703703703703e-3_f64 * t96181 - 0.17411041666666666666e-2_f64 * t96184;
    t97303
}
