//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 1095/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk1095<F: Float>(t1: F, t128: F, t2580: F, t33598: F, t350: F, t126: F, t15541: F, t190: F, t1903: F, t314: F, t442: F, t7953: F) -> (F, F) {
    let t33606 = t33598 * t2580 * t128 * t1 * t350;
    let t33614 = t7953 * t126 * t1903 * t15541 * t314 * t190 * t442;
    (t33606, t33614)
}
