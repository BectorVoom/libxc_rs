//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 960/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk960<F: Float>(t17612: F, t275: F, t176: F, t1006: F, t5471: F, t1584: F, t1567: F, t2325: F, t5242: F, t1442: F, t15067: F, t15066: F, sigma2: F) -> (F, F, F, F, F, F) {
    let t17613 = t17612 * t275;
    let t17615 = t176 * t17613 * sigma2;
    let t17618 = t1006 * t5471;
    let t17619 = t17618 * t1584;
    let t17622 = t2325 * t1567;
    let t17623 = t17622 * t5242;
    let t17626 = t15067 * t1442;
    let t17627 = t15066 * t17626;
    (t17615, t17618, t17619, t17622, t17623, t17627)
}
