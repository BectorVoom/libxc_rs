//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1413/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1413<F: Float>(t18367: F, t449: F, t446: F, t1659: F, t2794: F, t13045: F, t13048: F, t13050: F, t13053: F, t13055: F, t13057: F, t13060: F, t13094: F, t13096: F, t15795: F, t15798: F, t8524: F, t9272: F, t9313: F, t9315: F) -> F {
    let t18368 = t449 * t18367;
    let t18369 = t446 * t18368;
    let t18371 = t2794 * t1659;
    let t18373 = t8524 + t9315 + t13045 / F::new(8.0) - t9313 - t13048 / F::new(16.0) - t13050 / F::new(8.0) - t13053 / F::new(8.0) + t13055 / F::new(8.0) + t13057 / F::new(8.0) - t13060 / F::new(8.0) - t9272 + t13094 + t13096 - t15795 / F::new(16.0) - t15798 / F::new(16.0) - t18369 / F::new(16.0) - t18371 / F::new(8.0);
    t18373
}
