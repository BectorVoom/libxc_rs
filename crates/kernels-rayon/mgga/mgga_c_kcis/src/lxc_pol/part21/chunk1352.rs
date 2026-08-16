//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1352/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1352(t27055: f64, t28204: f64, t26955: f64, t26957: f64, t28107: f64, t28113: f64, t92795: f64, t95844: f64, t95850: f64, t96917: f64, t96926: f64, t96937: f64, t96942: f64, t96943: f64, t96945: f64) -> f64 {
    let t96946 = t28204 * t27055;
    let t96948 = 0.23168402777777777778e-3_f64 * t96917 * t26957 + 0.30918233506944444444e-4_f64 * t96926 * t26957 - 0.11607361111111111111e-1_f64 * t95844 - 0.61782407407407407408e-3_f64 * t92795 * t28107 - 0.61782407407407407408e-3_f64 * t92795 * t28113 + 0.46429444444444444443e-2_f64 * t95850 - 0.61836467013888888889e-4_f64 * t26955 * t96937 + t96942 + t96943 + t96945 + 0.30918233506944444444e-4_f64 * t96946;
    t96948
}
