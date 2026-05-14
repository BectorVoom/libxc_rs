//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 762/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk762<F: Float>(t1038: F, t2232: F, t10034: F, t3403: F, t2598: F, t3413: F, t829: F, t3438: F, t8786: F, t9894: F, t9897: F, t3120: F, t3402: F, t1086: F, t2628: F, t2233: F, t2982: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10035 = t1038 * t2232;
    let t10036 = t10034 * t10035;
    let t10037 = t3403 * t10036;
    let t10039 = t2598 * t3413;
    let t10040 = t829 * t10039;
    let t10041 = t3438 * t10040;
    let t10043 = t9894 * t8786;
    let t10044 = t10043 * t9897;
    let t10046 = t3402 * t3120;
    let t10047 = t1086 * t2628;
    let t10048 = t10046 * t10047;
    let t10050 = t2982 * t2233;
    (t10036, t10037, t10039, t10041, t10043, t10044, t10047, t10048, t10050)
}
