//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1285/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1285<F: Float>(t32159: F, t9442: F, t110077: F, t32052: F, t10349: F, t31883: F, t15461: F, t9358: F, t3441: F, t9390: F, t12671: F, t31848: F, t12769: F, t2689: F, t982: F, t12705: F, t9352: F) -> (F, F, F, F, F, F, F, F, F) {
    let t110756 = t32159 * t9442;
    let t110762 = 0.73697530864197530862e-3 * t110077;
    let t110778 = t32052 * t9442;
    let t110815 = 6.0 * t31883 * t10349;
    let t110817 = 3.0 * t15461 * t9358;
    let t110821 = t9390 * t3441;
    let t110824 = t12671 * t31848;
    let t110827 = t982 * t2689 * t12769;
    let t110829 = t12705 * t9352;
    (t110756, t110762, t110778, t110815, t110817, t110821, t110824, t110827, t110829)
}
