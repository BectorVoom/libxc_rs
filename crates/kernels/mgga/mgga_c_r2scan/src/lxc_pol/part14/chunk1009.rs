//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1009/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1009<F: Float>(t132: F, t1567: F, t39613: F, t7340: F, t1054: F, t6132: F, t7345: F, t6139: F, t10872: F, t11686: F, t10891: F, t11748: F, t10760: F, t19877: F, t25562: F, t261: F, t3304: F, t7233: F) -> (F, F, F, F, F, F, F) {
    let t39614 = t132 * t1567;
    let t39616 = t39613 * t39614 * t7340;
    let t39619 = t6132 * t1054 * t7345;
    let t39622 = t6139 * t1054 * t7340;
    let t39627 = t10872 * t11686;
    let t39629 = t11748 * t10891;
    let t39632 = t19877 * t10760 * t25562;
    let t39635 = t3304 * t261 * t7233;
    (t39616, t39619, t39622, t39627, t39629, t39632, t39635)
}
