//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1159/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1159<F: Float>(t102137: F, t102142: F, t102151: F, t102155: F, t102158: F, t20882: F, t27583: F, t28701: F, t28758: F, t6159: F, t7968: F, t7978: F, t8213: F, t95001: F, t99233: F, t99282: F, t99293: F, t99565: F) -> (F,) {
    let t102164 = 0.30918233506944444445e-4 * t99565 * t28701 - 0.46336805555555555556e-3 * t27583 * t102137 - t99282 + 0.25742669753086419753e-4 * t95001 + 0.34752604166666666667e-3 * t7978 * t102142 - 0.24734586805555555556e-3 * t99233 * t8213 + 0.46377350260416666667e-4 * t7968 * t102142 + 0.11584201388888888889e-3 * t27583 * t102151 - 0.30952962962962962962e-2 * t102155 + t99293 - 0.38691203703703703703e-2 * t102158 + 0.23168402777777777778e-3 * t27583 * t6159 * t28758 * t20882;
    (t102164,)
}
