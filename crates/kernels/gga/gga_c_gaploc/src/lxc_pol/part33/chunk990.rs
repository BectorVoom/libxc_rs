//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 990/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk990<F: Float>(t15665: F, t15672: F, t27846: F, t4066: F, t92: F, t27842: F, t5345: F, t5348: F, t1695: F, t3220: F, t1699: F, t3225: F, t7102: F, t871: F, t2628: F, t7340: F) -> (F, F, F, F, F, F) {
    let t27853 = t15672 * t4066 * t27846 * t15665 * t92;
    let t27856 = t5345 * t27842 * t5348;
    let t27858 = t3220 * t1695;
    let t27860 = t3225 * t1699;
    let t27868 = t7102 * t871;
    let t28022 = 0.11916829983950142223e0 * t7340 * t2628;
    (t27853, t27856, t27858, t27860, t27868, t28022)
}
