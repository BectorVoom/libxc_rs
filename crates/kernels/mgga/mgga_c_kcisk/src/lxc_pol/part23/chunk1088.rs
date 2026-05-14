//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1088/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1088<F: Float>(t2097: F, t3696: F, t14736: F, t14752: F, t14757: F, t14798: F, t14804: F, t19644: F, t21839: F, t21842: F, t21845: F, t21848: F, t21852: F, t21855: F, t21859: F, t21866: F, t21869: F, t4436: F, t4438: F, t4461: F, t4471: F, t4472: F, t6523: F, t6545: F) -> (F,) {
    let t21872 = t2097 * t3696;
    let t21875 = -0.23392893589820816284e1 * t4471 * t21839 - 0.11696446794910408142e1 * t4471 * t21842 - 0.1038945353962551798e3 * t14736 * t21845 - 0.19298809906722418785e3 * t14757 * t21848 + 0.64329366355741395948e2 * t4461 * t21852 + 0.32164683177870697974e2 * t4461 * t21855 + 0.20691336878655965246e4 * t14798 * t21859 - 4.0 * t14804 * t6523 + 0.64329366355741395948e2 * t14752 * t6545 - 4.0 * t4436 * t21866 - 2.0 * t21869 * t4438 - 0.11696446794910408142e1 * t21872 * t4472 + t19644;
    (t21875,)
}
