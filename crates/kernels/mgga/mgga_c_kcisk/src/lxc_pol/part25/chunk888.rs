//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 888/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk888<F: Float>(t11208: F, t2529: F, t2538: F, t2534: F, t3805: F, t2533: F, t5176: F, t415: F, t2372: F, t4624: F, t1648: F, t6771: F, t4652: F, t2365: F, t2869: F) -> (F, F, F, F, F, F, F, F) {
    let t15951 = t11208 * t2529;
    let t15953 = t11208 * t2538;
    let t15955 = t3805 * t2534;
    let t15957 = t5176 * t2533;
    let t15958 = t415 * t15957;
    let t15970 = t2372 * t4624;
    let t15975 = t6771 * t1648;
    let t15978 = t2372 * t4652;
    let t15989 = t2869 * t2365;
    (t15951, t15953, t15955, t15958, t15970, t15975, t15978, t15989)
}
