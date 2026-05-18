//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 839/1327 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk839<F: Float>(t3406: F, t7115: F, t9921: F, t10029: F, t2598: F, t3404: F, t1038: F, t2232: F, t3403: F, t3413: F, t829: F, t3438: F) -> (F, F, F, F, F, F) {
    let t10030 = t7115 * t3406;
    let t10031 = t9921 * t10030;
    let t10032 = t10029 * t10031;
    let t10034 = t3404 * t2598;
    let t10035 = t1038 * t2232;
    let t10036 = t10034 * t10035;
    let t10037 = t3403 * t10036;
    let t10039 = t2598 * t3413;
    let t10040 = t829 * t10039;
    let t10041 = t3438 * t10040;
    (t10031, t10032, t10036, t10037, t10039, t10041)
}
