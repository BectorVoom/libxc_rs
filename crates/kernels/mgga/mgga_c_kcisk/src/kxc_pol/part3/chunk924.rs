//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 924/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk924<F: Float>(t13665: F, t347: F, t355: F, t13633: F, t13522: F, t13533: F, t13536: F, t13540: F, t13543: F, t13549: F, t13555: F, t13650: F, t13653: F, t13656: F, t13659: F, t13662: F) -> (F, F) {
    let t13666 = F::new(0.73028148148148148147e0) * t13665;
    let t13669 = F::new(1.0) / t347 / t355 / F::new(8.0);
    let t13670 = t13669 * t13633;
    let t13672 = F::new(0.93011851851851851854e0) * t13522;
    let t13673 = -F::new(0.59793333333333333333e0) * t13533 + F::new(0.29896666666666666667e0) * t13536 - F::new(0.33218518518518518518e0) * t13540 + F::new(0.11958666666666666667e1) * t13543 - F::new(0.17938e1) * t13549 - F::new(0.29896666666666666667e0) * t13555 + F::new(0.32862666666666666666e0) * t13650 - F::new(0.28483875e1) * t13653 + F::new(0.46074375e0) * t13656 - F::new(0.16431333333333333333e0) * t13659 + F::new(0.98587999999999999998e0) * t13662 - t13666 + F::new(0.142419375e1) * t13670 - t13672;
    (t13670, t13673)
}
