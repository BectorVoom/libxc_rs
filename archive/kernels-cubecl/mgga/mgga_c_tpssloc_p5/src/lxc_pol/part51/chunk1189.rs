//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1189/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1189<F: Float>(t5: F, t31688: F, t8515: F, t1862: F, t79: F, t641: F, t8513: F, t31019: F, t31672: F, t31675: F, t31677: F, t31681: F, t31684: F, t8512: F) -> (F, F, F, F) {
    let t7 = piecewise3::<F>(F::cast_from(0.0_f64) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t31690 = F::cast_from(5.0_f64) / F::cast_from(27.0_f64) * t31688 * t8515;
    let t31691 = t79 * t1862;
    let t31693 = t8513 * t31691 * t641;
    let t31699 = piecewise3::<F>(t8, F::cast_from(0.0_f64), -F::cast_from(5.0_f64) / F::cast_from(72.0_f64) * t31672 * t8515 + F::cast_from(5.0_f64) / F::cast_from(12.0_f64) * t31675 * t31677 + F::cast_from(5.0_f64) / F::cast_from(18.0_f64) * t31681 * t31684 + t31690 - F::cast_from(5.0_f64) / F::cast_from(36.0_f64) * t8512 * t31693 - F::cast_from(5.0_f64) / F::cast_from(72.0_f64) * t8512 * t31019);
    (t31690, t31691, t31693, t31699)
}
