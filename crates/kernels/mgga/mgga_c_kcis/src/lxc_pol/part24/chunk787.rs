//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 787/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk787<F: Float>(t1068: F, t1646: F, t10108: F, t1030: F, t3073: F, t1072: F, t4833: F, t331: F, t4837: F, t1035: F, t167: F, t4845: F) -> (F, F, F, F, F, F, F) {
    let t13598 = t1068 * t1646;
    let t13600 = t10108 * t1646;
    let t13658 = t1030 * t3073;
    let t13665 = F::cast_from(0.93706135855523581992e-2_f64) * t1072 * t4833;
    let t13667 = F::cast_from(0.93706135855523581992e-2_f64) * t331 * t4837;
    let t13677 = t1035 * t167;
    let t13682 = F::cast_from(0.93706135855523581992e-2_f64) * t331 * t4845;
    (t13598, t13600, t13658, t13665, t13667, t13677, t13682)
}
