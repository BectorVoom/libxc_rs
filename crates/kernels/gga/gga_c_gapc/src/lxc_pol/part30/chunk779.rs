//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 779/1135 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk779<F: Float>(t10309: F, t2531: F, t799: F, t2493: F, t435: F, t3243: F, t2316: F, t493: F, t3230: F, t2225: F, t3198: F, t2217: F, t10203: F, t2456: F, t3258: F, t3253: F, t6948: F) -> (F, F, F, F, F, F, F) {
    let t10310 = t10309 * t2531;
    let t10311 = t799 * t10310;
    let t10313 = t435 * t2493;
    let t10314 = t3243 * t10313;
    let t10316 = t493 * t2316;
    let t10317 = t3230 * t10316;
    let t10319 = t2225 * t3198;
    let t10321 = t2217 * t3198;
    let t10325 = t10203 * t2456;
    let t10326 = t3258 * t10325;
    let t10328 = t3253 * t6948;
    (t10311, t10314, t10317, t10319, t10321, t10326, t10328)
}
