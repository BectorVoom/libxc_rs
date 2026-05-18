//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 921/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk921<F: Float>(t2354: F, t88: F, t66: F, t673: F, t2680: F, t844: F, t4620: F, t4714: F, t8594: F, t8596: F, t8598: F, t8691: F, t8693: F, t8695: F) -> (F, F, F, F, F) {
    let t8850 = t88 * t2354;
    let t8858 = t66 * t673;
    let t8862 = t88 * t2680;
    let t8866 = t66 * t844;
    let t8881 = -F::new(0.47063e1) * t8594 + F::new(0.31375333333333333334e1) * t8596 - F::new(0.36604555555555555556e1) * t8598 - F::new(0.16068111111111111111e1) * t4620 + F::new(0.28051666666666666666e0) * t8691 - F::new(0.56103333333333333332e0) * t8693 - F::new(0.6545388888888888889e0) * t8695 - F::new(0.46308888888888888888e0) * t4714;
    (t8850, t8858, t8862, t8866, t8881)
}
