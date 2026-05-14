//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 737/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk737<F: Float>(t118: F, t13588: F, t4882: F, t1737: F, t2471: F, t1742: F, t2475: F, t1734: F, t2466: F, t1068: F, t1646: F, t10108: F, t1030: F, t3073: F, t1072: F, t4833: F) -> (F, F, F, F, F, F, F, F) {
    let t13589 = t118 * t13588;
    let t13590 = t13589 * t4882;
    let t13592 = t2471 * t1737;
    let t13594 = t2475 * t1742;
    let t13596 = t2466 * t1734;
    let t13598 = t1068 * t1646;
    let t13600 = t10108 * t1646;
    let t13658 = t1030 * t3073;
    let t13665 = 0.93706135855523581992e-2 * t1072 * t4833;
    (t13590, t13592, t13594, t13596, t13598, t13600, t13658, t13665)
}
