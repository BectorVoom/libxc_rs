//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1315/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1315(t1598: f64, t251: f64, t62417: f64, t101943: f64, t27595: f64, t29266: f64, t4142: f64, t29578: f64, t7974: f64, t102250: f64, t102438: f64, t27583: f64, t28772: f64, t28853: f64, t7971: f64, t7986: f64, t8213: f64, t98632: f64, t99219: f64, t99512: f64, t99524: f64) -> (f64, f64) {
    let t102543 = t62417 * t251 * t1598;
    let t102546 = t27595 * t101943;
    let t102548 = t4142 * t29266;
    let t102554 = t29578 * t7974;
    let t102558 = -0.18534722222222222222e-2_f64 * t99219 * t8213 + 0.34752604166666666667e-3_f64 * t102250 * t7986 + 0.34752604166666666667e-3_f64 * t102250 * t7971 + 0.46377350260416666667e-4_f64 * t102543 * t7971 - 0.30945286961263020834e-5_f64 * t102546 + 0.15476481481481481481e-2_f64 * t102548 - 0.30952962962962962962e-2_f64 * t98632 + t99512 + 0.23168402777777777778e-3_f64 * t27583 * t102438 + 0.15445601851851851852e-3_f64 * t99524 - 0.11584201388888888889e-3_f64 * t102554 - 0.24734586805555555556e-3_f64 * t28853 * t28772;
    (t102548, t102558)
}
