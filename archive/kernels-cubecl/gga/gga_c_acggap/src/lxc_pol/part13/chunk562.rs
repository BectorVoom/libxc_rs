//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 562/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk562<F: Float>(t441: F, t851: F, t323: F, t1222: F, t857: F, t852: F, t872: F, t1221: F, t322: F, t1220: F, t316: F, t3101: F, t317: F) -> (F, F, F, F, F, F) {
    let t3901 = t851 * t441;
    let t3902 = t3901 * t323;
    let t3904 = t857 * t1222;
    let t3906 = t852 * t872;
    let t3908 = t322 * t1221;
    let t3909 = t1220 * t3908;
    let t3910 = t316 * t3909;
    let t3912 = t317 * t3101;
    (t3902, t3904, t3906, t3909, t3910, t3912)
}
