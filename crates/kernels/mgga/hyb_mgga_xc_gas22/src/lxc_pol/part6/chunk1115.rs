//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1115/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1115<F: Float>(t20685: F, t2655: F, t2654: F, t16: F, t7940: F, t1033: F, t15: F, t221: F, t439: F, t12: F, t21862: F, t222: F, t1056: F, t7349: F, t2666: F, t1040: F) -> (F, F, F, F, F, F, F, F) {
    let t21868 = t2655 * t20685;
    let t21869 = t2654 * t21868;
    let t21871 = t16 * t7940;
    let t21872 = t1033 * t21871;
    let t21874 = t15 * t7940;
    let t21875 = t221 * t21874;
    let t21877 = f64::powf(t439, -0.25e1);
    let t21880 = t21877 * t12 * t21862 * t222;
    let t21882 = t7349 * t1056;
    let t21884 = t2666 * t21868;
    let t21886 = t1040 * t21871;
    (t21869, t21872, t21874, t21875, t21880, t21882, t21884, t21886)
}
