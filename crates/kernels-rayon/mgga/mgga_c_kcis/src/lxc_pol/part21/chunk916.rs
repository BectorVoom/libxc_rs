//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 916/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk916(t22: f64, t4864: f64, t4715: f64, t13710: f64, t13712: f64, t13723: f64, t13732: f64, t13767: f64, t13939: f64, t13942: f64, t13945: f64, t9726: f64, t9729: f64) -> (f64, f64) {
    let t13948 = t22 * t4864;
    let t13949 = t13948 * t4715;
    let t13951 = 0.13287407407407407408e0_f64 * t13712 - t13939 + 0.11958666666666666667e1_f64 * t13723 - 0.17938e1_f64 * t13732 - t9726 - t9729 + 0.3071625e0_f64 * t13942 + 0.1898925e1_f64 * t13767 - 0.91285185185185185185e-1_f64 * t13945 - 0.13287407407407407408e0_f64 * t13710 + 0.71202444444444444443e0_f64 * t13949;
    (t13949, t13951)
}
