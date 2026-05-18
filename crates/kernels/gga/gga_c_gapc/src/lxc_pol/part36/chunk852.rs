//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 852/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk852<F: Float>(t2431: F, t3197: F, t10215: F, t6791: F, t831: F, t1062: F, t2200: F, t2212: F, t3254: F, t2951: F, t760: F, t2208: F) -> (F, F, F, F) {
    let t10216 = t3197 * t2431;
    let t10217 = t10215 * t10216;
    let t10219 = t6791 * t831;
    let t10220 = t1062 * t10219;
    let t10222 = t2200 * t2212;
    let t10223 = t3254 * t10222;
    let t10225 = t2951 * t760;
    let t10226 = t10225 * t2208;
    (t10217, t10220, t10223, t10226)
}
