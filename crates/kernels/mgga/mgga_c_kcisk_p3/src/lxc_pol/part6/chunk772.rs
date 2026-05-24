//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 772/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk772<F: Float>(t2454: F, t5191: F, t1755: F, t11208: F, t2529: F, t2538: F, t2534: F, t3805: F, t2365: F, t2869: F) -> (F, F, F, F, F, F) {
    let t15862 = t5191 * t2454;
    let t15936 = t2454 * t1755;
    let t15951 = t11208 * t2529;
    let t15953 = t11208 * t2538;
    let t15955 = t3805 * t2534;
    let t15989 = t2869 * t2365;
    (t15862, t15936, t15951, t15953, t15955, t15989)
}
