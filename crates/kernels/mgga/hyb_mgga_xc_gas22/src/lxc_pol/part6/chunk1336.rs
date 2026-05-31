//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1336/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1336<F: Float>(t20838: F, t4143: F, t2189: F, t2234: F, t4140: F, t10703: F, t2311: F, t3352: F, t2188: F, t810: F, t25146: F, t8618: F) -> (F, F, F, F, F, F) {
    let t29068 = F::cast_from(0.16081979498692535067e2_f64) * t20838 * t4143;
    let t29071 = F::cast_from(6.0_f64) * t2234 * t4140 * t2189;
    let t29072 = t2311 * t10703;
    let t29076 = t3352 * t3352;
    let t29079 = F::cast_from(4.0_f64) * t2188 * t29076 * t810;
    let t29081 = F::cast_from(0.38596750796862084161e3_f64) * t25146 * t8618;
    (t29068, t29071, t29072, t29076, t29079, t29081)
}
