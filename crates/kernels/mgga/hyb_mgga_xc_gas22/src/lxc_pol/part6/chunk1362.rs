//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1362/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1362<F: Float>(t25270: F, t8986: F, t8993: F, t9104: F, t25262: F, t8997: F, t2515: F, t2521: F, t4244: F, t2479: F, t4273: F, t7148: F) -> (F, F, F, F, F) {
    let t29656 = F::cast_from(0.19298375398431042081e3_f64) * t25270 * t8986;
    let t29658 = F::cast_from(0.32163958997385070134e2_f64) * t9104 * t8993;
    let t29660 = F::cast_from(0.1034520258385468006e4_f64) * t25262 * t8997;
    let t29663 = F::new(6.0) * t2521 * t4244 * t2515;
    let t29666 = F::cast_from(0.57895126195293126241e3_f64) * t7148 * t4273 * t2479;
    (t29656, t29658, t29660, t29663, t29666)
}
