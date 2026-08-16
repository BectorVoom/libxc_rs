//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 740/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk740(t2260: f64, t7968: f64, t7976: f64, t7978: f64, t7991: f64, t8166: f64, t8169: f64, t8172: f64, t8177: f64, t8180: f64, t8209: f64, t8213: f64, t8218: f64, t8222: f64, t8226: f64) -> f64 {
    let t8236 = -0.34752604166666666667e-3_f64 * t8209 * t2260 + 0.46377350260416666667e-4_f64 * t7968 * t8213 + 0.92673611111111111112e-3_f64 * t8218 * t2260 - t7976 - 0.11584201388888888889e-3_f64 * t7978 * t8222 + 0.34752604166666666667e-3_f64 * t7978 * t8226 + 0.34752604166666666667e-3_f64 * t7978 * t8213 + t7991 + 0.11607361111111111111e-2_f64 * t8166 + 0.17411041666666666666e-2_f64 * t8169 - 0.17411041666666666666e-2_f64 * t8172 - 0.46429444444444444443e-2_f64 * t8177 + 0.11607361111111111111e-2_f64 * t8180;
    t8236
}
