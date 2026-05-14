//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 863/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk863<F: Float>(t227: F, t3288: F, t967: F, t10447: F, t79: F, t918: F, t3122: F, t5: F, t156: F, t5822: F, t1048: F, t23: F, t6: F, t161: F, t3269: F, t9: F) -> (F, F, F, F, F, F, F, F) {
    let t15783 = 1.0 / t3288 / t227;
    let t15821 = 2.0 * t967;
    let t15822 = 6.0 * t10447;
    let t15868 = t918 * t79;
    let t15995 = t5 * t3122;
    let t16210 = t156 * t5822;
    let t16214 = 1.0 / t23 / t1048;
    let t16215 = t6 * t16214;
    let t16216 = t161 * t16215;
    let t16220 = 1.0 / t9 / t3269;
    (t15783, t15821, t15822, t15868, t15995, t16210, t16216, t16220)
}
