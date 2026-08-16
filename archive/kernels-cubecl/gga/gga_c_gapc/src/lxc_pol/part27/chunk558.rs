//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 558/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk558<F: Float>(t126: F, t820: F, t284: F, t1063: F, t828: F, t3188: F, t876: F, t277: F, t2902: F) -> (F, F, F, F, F, F) {
    let t3201 = t126 * t820;
    let t3202 = t284 * t3201;
    let t3204 = t828 * t1063;
    let t3206 = t3188 * t876;
    let t3207 = t284 * t3206;
    let t3209 = t2902 * t277;
    (t3201, t3202, t3204, t3206, t3207, t3209)
}
