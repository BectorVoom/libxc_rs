//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1396/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1396<F: Float>(t33766: F, t9859: F, t109626: F, t110068: F, t115416: F, t119580: F, t119608: F, t119614: F, t120207: F, t120285: F, t120376: F, t2737: F, t27726: F, t32339: F, t33928: F, t33961: F, t35012: F, t35018: F, t9519: F, t9536: F, t9855: F) -> (F,) {
    let t120723 = t33766 * t9859;
    let t120735 = 0.34822083333333333332e-2 * t119580 - 0.52083333333333333333e-2 * t9536 * t120376 - 0.10416666666666666667e-1 * t9536 * t120285 - 0.92592592592592592592e-2 * t32339 * t35012 + 0.10317654320987654321e-2 * t119608 + 0.52083333333333333333e-2 * t2737 * t120207 + 0.10416666666666666667e-1 * t35018 * t9519 + 0.40208333333333333335e-2 * t120723 * t9519 + 0.10416666666666666667e-1 * t33928 * t9855 + 0.10416666666666666667e-1 * t33961 * t9855 - 0.34822083333333333332e-2 * t119614 + 0.38691203703703703703e-3 * t110068 - 0.69444444444444444444e-2 * t109626 * t115416 * t27726;
    (t120735,)
}
