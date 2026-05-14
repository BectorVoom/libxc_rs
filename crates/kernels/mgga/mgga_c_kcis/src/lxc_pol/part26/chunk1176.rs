//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1176/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1176<F: Float>(t1598: F, t251: F, t62417: F, t101943: F, t27595: F, t29266: F, t4142: F, t29578: F, t7974: F, t102250: F, t102438: F, t27583: F, t28772: F, t28853: F, t7971: F, t7986: F, t8213: F, t98632: F, t99219: F, t99512: F, t99524: F) -> (F, F) {
    let t102543 = t62417 * t251 * t1598;
    let t102546 = t27595 * t101943;
    let t102548 = t4142 * t29266;
    let t102554 = t29578 * t7974;
    let t102558 = -0.18534722222222222222e-2 * t99219 * t8213 + 0.34752604166666666667e-3 * t102250 * t7986 + 0.34752604166666666667e-3 * t102250 * t7971 + 0.46377350260416666667e-4 * t102543 * t7971 - 0.30945286961263020834e-5 * t102546 + 0.15476481481481481481e-2 * t102548 - 0.30952962962962962962e-2 * t98632 + t99512 + 0.23168402777777777778e-3 * t27583 * t102438 + 0.15445601851851851852e-3 * t99524 - 0.11584201388888888889e-3 * t102554 - 0.24734586805555555556e-3 * t28853 * t28772;
    (t102548, t102558)
}
