//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1425/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1425<F: Float>(t32473: F, t9859: F, t56817: F, t79: F, t2736: F, t33960: F, t9515: F, t32401: F, t33767: F, t109531: F, t109543: F, t109801: F, t109806: F, t114251: F, t114254: F, t32359: F, t32380: F, t32385: F, t32480: F, t33778: F, t9519: F, t9855: F, t9869: F) -> (F,) {
    let t115589 = t32473 * t9859;
    let t115592 = t56817 * t79;
    let t115593 = t115592 * t2736;
    let t115596 = t9515 * t33960;
    let t115606 = 0.13402777777777777778e-2 * t33767 * t32401;
    let t115609 = -0.27777777777777777778e-1 * t32359 * t9855 + 0.61905925925925925925e-2 * t114251 - 0.10317654320987654321e-1 * t114254 - 0.27777777777777777778e-1 * t32480 * t9869 - 0.60312500000000000001e-2 * t33778 * t32380 + 0.40208333333333333334e-2 * t115589 * t9519 - 0.116403125e-2 * t115593 * t32380 + 0.40208333333333333334e-2 * t115596 * t9519 + 0.20104166666666666667e-2 * t109531 * t9855 + 0.40208333333333333334e-2 * t109543 * t9855 + 0.20104166666666666667e-2 * t33778 * t32385 + t115606 - 0.34722222222222222222e-2 * t109801 + 0.13402777777777777778e-2 * t109806;
    (t115609,)
}
