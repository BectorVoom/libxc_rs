//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1002/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1002<F: Float>(t25412: F, t2812: F, t5025: F, t4933: F, t7274: F, t930: F, t4929: F, t2619: F, t2751: F, t4997: F, t2758: F, t5002: F, t5007: F, t7878: F, t940: F, t5021: F, t913: F) -> (F, F, F, F, F, F, F) {
    let t42427 = t2812 * t25412 * t5025;
    let t42487 = t930 * t7274 * t4933;
    let t42490 = t930 * t7274 * t4929;
    let t42743 = t2751 * t2619 * t4997;
    let t42785 = t2758 * t2619 * t5002;
    let t42878 = t940 * t7878 * t5007;
    let t42991 = t913 * t7274 * t5021;
    (t42427, t42487, t42490, t42743, t42785, t42878, t42991)
}
