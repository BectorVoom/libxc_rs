//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 694/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk694<F: Float>(t16177: F, t1910: F, t1909: F, t18: F, t979: F, t432: F, t452: F, t4623: F, t4613: F, t8392: F, t11535: F, t11537: F, t11549: F, t11550: F, t11593: F, t16147: F, t16152: F, t16157: F, t16162: F, t16166: F, t16171: F, t16174: F, t1901: F, t446: F) -> (F,) {
    let t16178 = t1910 * t16177;
    let t16179 = t1909 * t16178;
    let t16182 = t18 * t979;
    let t16183 = t1910 * t16182;
    let t16184 = t1909 * t16183;
    let t16188 = t452 * t4623 * t432;
    let t16192 = t8392 * t4613;
    let t16194 = -4.0 / 3.0 * t1901 * t16147 + 4.0 / 9.0 * t1901 * t16152 - 2.0 / 9.0 * t1901 * t16157 - 2.0 / 9.0 * t1901 * t16162 - 2.0 / 3.0 * t1901 * t16166 + 8.0 / 9.0 * t11593 * t16171 + 2.0 / 9.0 * t1901 * t16174 + 2.0 / 9.0 * t1901 * t16179 + 4.0 / 9.0 * t11593 * t16184 + t11535 + t11537 - t446 * t16188 / 3.0 + t11549 - 8.0 / 27.0 * t11550 - 2.0 / 27.0 * t16192;
    (t16194,)
}
