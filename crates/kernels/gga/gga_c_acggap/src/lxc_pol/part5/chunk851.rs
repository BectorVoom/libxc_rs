//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 851/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk851<F: Float>(t14047: F, t3367: F, t3378: F, t3402: F, t3077: F, t3371: F, t1167: F, t10098: F, t1172: F) -> (F, F, F, F, F) {
    let t14048 = t14047 * t3367;
    let t14050 = t3378 * t3402;
    let t14053 = t3077 * t3371;
    let t14054 = t14053 * t1167;
    let t14056 = t10098 * t1172;
    (t14048, t14050, t14053, t14054, t14056)
}
