//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 700/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk700<F: Float>(t15762: F, t233: F, t222: F, t3276: F, t227: F, t3288: F, t3180: F, t3463: F, t3275: F, t3188: F, t2454: F, t5183: F, t5191: F, t1755: F, t11208: F, t2529: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t15763 = t233 * t15762;
    let t15772 = 1.0 / t3276 / t222;
    let t15783 = 1.0 / t3288 / t227;
    let t15799 = 3.0 * t3180;
    let t15800 = 3.0 * t3463;
    let t15803 = 3.0 * t3275;
    let t15804 = 6.0 * t3188;
    let t15858 = t5183 * t2454;
    let t15862 = t5191 * t2454;
    let t15936 = t2454 * t1755;
    let t15951 = t11208 * t2529;
    (t15763, t15772, t15783, t15799, t15800, t15803, t15804, t15858, t15862, t15936, t15951)
}
