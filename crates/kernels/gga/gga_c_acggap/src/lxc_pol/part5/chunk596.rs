//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 596/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk596<F: Float>(t316: F, t3912: F, t3044: F, t317: F, t863: F, t463: F, t864: F, t449: F, t180: F, t3035: F, t3037: F, t3242: F, t323: F, t868: F, t880: F, t3054: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t3914 = 0.65854491829355115987e0 * t316 * t3912;
    let t3915 = t317 * t3044;
    let t3917 = 0.39512695097613069591e1 * t863 * t3915;
    let t3918 = t864 * t463;
    let t3919 = t449 * t3918;
    let t3920 = t863 * t3919;
    let t3922 = t3035 * t180;
    let t3923 = t317 * t3037;
    let t3925 = 0.39512695097613069591e1 * t3922 * t3923;
    let t3930 = t3242 * t180;
    let t3932 = 0.19756347548806534796e1 * t3930 * t323;
    let t3935 = t868 * t880;
    let t3937 = t3054 * t180;
    (t3914, t3915, t3917, t3918, t3919, t3920, t3922, t3923, t3925, t3930, t3932, t3935, t3937)
}
