//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 561/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk561<F: Float>(t316: F, t3912: F, t3044: F, t317: F, t863: F, t463: F, t864: F, t449: F, t180: F, t3035: F, t3037: F, t3242: F) -> (F, F, F, F, F, F, F, F) {
    let t3914 = F::cast_from(0.65854491829355115987e0_f64) * t316 * t3912;
    let t3915 = t317 * t3044;
    let t3917 = F::cast_from(0.39512695097613069591e1_f64) * t863 * t3915;
    let t3918 = t864 * t463;
    let t3919 = t449 * t3918;
    let t3920 = t863 * t3919;
    let t3922 = t3035 * t180;
    let t3923 = t317 * t3037;
    let t3925 = F::cast_from(0.39512695097613069591e1_f64) * t3922 * t3923;
    let t3930 = t3242 * t180;
    (t3914, t3915, t3917, t3919, t3920, t3923, t3925, t3930)
}
