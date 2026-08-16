//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 557/1331 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk557<F: Float>(t435: F, t919: F, t3243: F, t1936: F, t2268: F, t831: F, t1062: F, t268: F, t2951: F, t2208: F, t2212: F, t829: F) -> (F, F, F, F, F, F, F, F, F) {
    let t3244 = t435 * t919;
    let t3245 = t3243 * t3244;
    let t3247 = t1936 * t919;
    let t3248 = t3243 * t3247;
    let t3250 = t2268 * t831;
    let t3251 = t1062 * t3250;
    let t3253 = t2951 * t268;
    let t3254 = t3253 * t2208;
    let t3255 = t829 * t2212;
    (t3244, t3245, t3247, t3248, t3250, t3251, t3253, t3254, t3255)
}
