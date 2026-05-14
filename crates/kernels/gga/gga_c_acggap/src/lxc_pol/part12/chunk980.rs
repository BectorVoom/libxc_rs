//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 980/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk980<F: Float>(t2041: F, t4632: F, t1426: F, t429: F, t598: F, t8539: F, t35500: F, t7380: F, t34050: F, t2095: F, t33901: F, t33884: F, t1998: F, t4503: F, t5124: F, t7647: F) -> (F, F, F, F, F, F, F, F) {
    let t35887 = t2041 * t4632;
    let t35907 = t598 * t1426 * t429 * t8539;
    let t35909 = t7380 * t35500;
    let t35911 = t7380 * t34050;
    let t35913 = t2095 * t33901;
    let t35915 = t2095 * t33884;
    let t35917 = t1998 * t4503;
    let t35919 = t7647 * t5124;
    (t35887, t35907, t35909, t35911, t35913, t35915, t35917, t35919)
}
