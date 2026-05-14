//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1153/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1153<F: Float>(t16110: F, t38921: F, t5674: F, t5675: F, t100417: F, t15951: F, t1901: F, t100411: F, t15955: F, t1882: F, t29653: F, t1871: F, t22952: F, t3103: F, t965: F, t116328: F, t3281: F, t7824: F) -> (F, F, F, F, F, F, F) {
    let t116387 = t5674 * t38921 * t5675 * t16110;
    let t116390 = t1901 * t100417 * t15951;
    let t116393 = t1901 * t100411 * t15955;
    let t116395 = t1882 * t29653;
    let t116396 = 2.0 / 9.0 * t116395;
    let t116400 = t22952 * t1871 * t5675 * t965 * t3103;
    let t116402 = t3281 * t7824 * t116328;
    (t116387, t116390, t116393, t116395, t116396, t116400, t116402)
}
