//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 285/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk285<F: Float>(t3874: F, t4002: F, t258: F, t3951: F, t1217: F, t2648: F, t1186: F, t2336: F, t89: F, t2857: F, t3691: F, t446: F, t1091: F, t824: F, t2665: F, t3700: F, t835: F) -> (F, F, F, F, F, F, F, F) {
    let t4003 = t3874 + t4002;
    let t4005 = t3951 * t258;
    let t4027 = t2648 * t1217;
    let t4032 = t89 * t2336 * t1186;
    let t4034 = t2857 * t3691;
    let t4035 = t446 * t4034;
    let t4037 = t1091 * t824;
    let t4038 = t2665 * t4037;
    let t4039 = t446 * t4038;
    let t4041 = t835 * t3700;
    (t4003, t4005, t4027, t4032, t4035, t4037, t4039, t4041)
}
