//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1181/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1181<F: Float>(t2744: F, t4534: F, t1620: F, t9571: F, t2748: F, t4565: F, t1610: F, t9555: F, t15084: F, t32231: F, t32232: F, t32233: F, t32237: F, t32309: F, t4530: F, t4535: F, t4536: F) -> (F, F, F, F, F) {
    let t32523 = t2744 * t4534;
    let t32526 = t9571 * t1620;
    let t32529 = t2748 * t4565;
    let t32533 = t9555 * t1610;
    let t32536 = -t15084 * t2748 - 2.0 * t1620 * t32533 + 2.0 * t32523 * t4536 + 4.0 * t32526 * t4535 + 2.0 * t32529 * t4535 - 2.0 * t4530 * t9571 - t32231 + t32232 + t32233 + t32237 + t32309;
    (t32523, t32526, t32529, t32533, t32536)
}
