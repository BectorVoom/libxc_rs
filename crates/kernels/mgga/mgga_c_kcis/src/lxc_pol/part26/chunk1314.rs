//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1314/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1314<F: Float>(t28727: F, t28781: F, t23157: F, t7967: F, t29354: F, t3728: F, t2260: F, t23198: F, t27607: F, t28835: F, t29514: F, t29550: F, t29569: F, t29599: F, t4409: F, t6176: F, t7971: F, t7978: F, t7984: F, t8226: F, t99219: F, t99504: F, t99506: F) -> (F, F) {
    let t102517 = t28727 * t28781;
    let t102526 = t7967 * t23157;
    let t102529 = t3728 * t29354;
    let t102535 = F::cast_from(0.34752604166666666667e-3_f64) * t27607 * t29550 + F::cast_from(0.34752604166666666667e-3_f64) * t7978 * t6176 * t7984 * t23198 - F::cast_from(0.61782407407407407407e-3_f64) * t102517 + t99504 - t99506 - F::cast_from(0.33980324074074074074e-2_f64) * t4409 * t29599 * t2260 - F::cast_from(0.69505208333333333334e-3_f64) * t27607 * t29514 - F::cast_from(0.69505208333333333334e-3_f64) * t27607 * t29569 + F::cast_from(0.45346742476851851852e-3_f64) * t102526 * t7971 - F::cast_from(0.15476481481481481481e-2_f64) * t102529 - F::cast_from(0.18534722222222222222e-2_f64) * t99219 * t8226 - F::cast_from(0.18534722222222222222e-2_f64) * t28727 * t28835;
    (t102529, t102535)
}
