//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1314/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1314(t28727: f64, t28781: f64, t23157: f64, t7967: f64, t29354: f64, t3728: f64, t2260: f64, t23198: f64, t27607: f64, t28835: f64, t29514: f64, t29550: f64, t29569: f64, t29599: f64, t4409: f64, t6176: f64, t7971: f64, t7978: f64, t7984: f64, t8226: f64, t99219: f64, t99504: f64, t99506: f64) -> (f64, f64) {
    let t102517 = t28727 * t28781;
    let t102526 = t7967 * t23157;
    let t102529 = t3728 * t29354;
    let t102535 = 0.34752604166666666667e-3_f64 * t27607 * t29550 + 0.34752604166666666667e-3_f64 * t7978 * t6176 * t7984 * t23198 - 0.61782407407407407407e-3_f64 * t102517 + t99504 - t99506 - 0.33980324074074074074e-2_f64 * t4409 * t29599 * t2260 - 0.69505208333333333334e-3_f64 * t27607 * t29514 - 0.69505208333333333334e-3_f64 * t27607 * t29569 + 0.45346742476851851852e-3_f64 * t102526 * t7971 - 0.15476481481481481481e-2_f64 * t102529 - 0.18534722222222222222e-2_f64 * t99219 * t8226 - 0.18534722222222222222e-2_f64 * t28727 * t28835;
    (t102529, t102535)
}
