//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1352/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1352<F: Float>(t27055: F, t28204: F, t26955: F, t26957: F, t28107: F, t28113: F, t92795: F, t95844: F, t95850: F, t96917: F, t96926: F, t96937: F, t96942: F, t96943: F, t96945: F) -> F {
    let t96946 = t28204 * t27055;
    let t96948 = F::cast_from(0.23168402777777777778e-3_f64) * t96917 * t26957 + F::cast_from(0.30918233506944444444e-4_f64) * t96926 * t26957 - F::cast_from(0.11607361111111111111e-1_f64) * t95844 - F::cast_from(0.61782407407407407408e-3_f64) * t92795 * t28107 - F::cast_from(0.61782407407407407408e-3_f64) * t92795 * t28113 + F::cast_from(0.46429444444444444443e-2_f64) * t95850 - F::cast_from(0.61836467013888888889e-4_f64) * t26955 * t96937 + t96942 + t96943 + t96945 + F::cast_from(0.30918233506944444444e-4_f64) * t96946;
    t96948
}
