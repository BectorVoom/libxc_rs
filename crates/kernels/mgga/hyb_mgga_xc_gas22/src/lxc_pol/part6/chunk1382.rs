//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1382/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1382<F: Float>(t3513: F, t2478: F, t968: F, t2477: F, t4238: F, t2480: F, t2521: F, t2523: F, t25468: F, t8990: F, t2479: F, t4244: F, t7075: F) -> (F, F, F, F, F) {
    let t29993 = t3513 * t3513;
    let t29996 = F::new(4.0) * t2478 * t29993 * t968;
    let t29997 = t4238 * t2477;
    let t29999 = F::new(2.0) * t29997 * t2480;
    let t30002 = F::new(0.32163958997385070134e2) * t2521 * t29993 * t2523;
    let t30004 = F::new(0.38596750796862084161e3) * t25468 * t8990;
    let t30007 = F::new(24.0) * t7075 * t4244 * t2479;
    (t29996, t29999, t30002, t30004, t30007)
}
