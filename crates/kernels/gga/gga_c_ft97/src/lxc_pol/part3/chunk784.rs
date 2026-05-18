//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 784/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk784<F: Float>(t16169: F, t3187: F, t1909: F, t4612: F, t8506: F, t3255: F, t920: F, t1910: F, t18: F, t979: F, t432: F, t452: F, t4623: F) -> (F, F, F, F, F) {
    let t16170 = t3187 * t16169;
    let t16171 = t1909 * t16170;
    let t16174 = t8506 * t4612;
    let t16177 = t920 * t3255;
    let t16178 = t1910 * t16177;
    let t16179 = t1909 * t16178;
    let t16182 = t18 * t979;
    let t16183 = t1910 * t16182;
    let t16184 = t1909 * t16183;
    let t16188 = t452 * t4623 * t432;
    (t16171, t16174, t16179, t16184, t16188)
}
