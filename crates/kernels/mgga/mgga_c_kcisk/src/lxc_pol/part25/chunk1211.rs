//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1211/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1211<F: Float>(t37229: F, t9340: F, t10335: F, t2685: F, t10337: F, t10349: F, t31883: F, t15461: F, t9358: F, t3441: F, t9390: F, t12671: F, t31848: F, t12769: F, t2689: F, t982: F) -> (F, F, F, F, F, F, F) {
    let t109162 = 6.0 * t37229 * t9340;
    let t109163 = t2685 * t10335;
    let t109165 = 6.0 * t109163 * t10337;
    let t110815 = 6.0 * t31883 * t10349;
    let t110817 = 3.0 * t15461 * t9358;
    let t110821 = t9390 * t3441;
    let t110824 = t12671 * t31848;
    let t110827 = t982 * t2689 * t12769;
    (t109162, t109165, t110815, t110817, t110821, t110824, t110827)
}
