//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1022/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1022<F: Float>(t24789: F, t5166: F, t1131: F, t6947: F, t729: F, t4965: F, t6074: F, t9803: F, t24737: F, t5073: F, t13885: F, t14159: F, t6848: F, t1091: F, t28123: F, t2599: F) -> (F, F, F, F, F, F, F, F, F) {
    let t31170 = t24789 * t5166;
    let t31175 = t729 * t6947 * t1131;
    let t31178 = t6074 * t4965;
    let t31179 = t9803 * t31178;
    let t31182 = t24737 * t5073;
    let t31183 = t13885 * t31182;
    let t31186 = t14159 * t6848;
    let t31189 = t28123 * t1091;
    let t31190 = t2599 * t31189;
    (t31170, t31175, t31178, t31179, t31182, t31183, t31186, t31189, t31190)
}
