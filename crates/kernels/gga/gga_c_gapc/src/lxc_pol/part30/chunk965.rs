//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 965/1135 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk965<F: Float>(t1: F, t128: F, t2580: F, t33598: F, t350: F, t126: F, t15541: F, t190: F, t1903: F, t314: F, t442: F, t7953: F, t11923: F, t11927: F, t3363: F, t1461: F, t8710: F) -> (F, F, F, F) {
    let t33606 = t33598 * t2580 * t128 * t1 * t350;
    let t33614 = t7953 * t126 * t1903 * t15541 * t314 * t190 * t442;
    let t33617 = t3363 * t11923 * t11927;
    let t33619 = t1461 * t8710;
    (t33606, t33614, t33617, t33619)
}
