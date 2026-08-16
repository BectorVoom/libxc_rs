//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 816/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk816<F: Float>(t3104: F, t5311: F, t3117: F, t12075: F, t5324: F, t3116: F, t5328: F, t8487: F, t3133: F, t3132: F, t1442: F, t4356: F) -> (F, F, F, F, F, F, F, F) {
    let t15236 = t3104 * t5311;
    let t15240 = t3117 * t5311;
    let t15254 = t12075 * t5324;
    let t15255 = t3116 * t15254;
    let t15270 = t8487 * t5328;
    let t15271 = t15270 * t3133;
    let t15272 = t3132 * t15271;
    let t15274 = t4356 * t1442;
    (t15236, t15240, t15254, t15255, t15270, t15271, t15272, t15274)
}
