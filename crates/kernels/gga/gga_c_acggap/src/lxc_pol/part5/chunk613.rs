//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 613/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk613<F: Float>(t1220: F, t3908: F, t316: F, t3101: F, t317: F, t3044: F, t863: F, t463: F, t864: F, t449: F, t180: F, t3035: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t3909 = t1220 * t3908;
    let t3910 = t316 * t3909;
    let t3912 = t317 * t3101;
    let t3914 = F::new(0.65854491829355115987e0) * t316 * t3912;
    let t3915 = t317 * t3044;
    let t3917 = F::new(0.39512695097613069591e1) * t863 * t3915;
    let t3918 = t864 * t463;
    let t3919 = t449 * t3918;
    let t3920 = t863 * t3919;
    let t3922 = t3035 * t180;
    (t3909, t3910, t3912, t3914, t3915, t3917, t3918, t3919, t3920, t3922)
}
