//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 886/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk886<F: Float>(t11522: F, t5541: F, t8677: F, t5462: F, t8681: F, t3670: F, t620: F, t190: F, t8448: F, t1: F, t116: F, t3703: F, t612: F, t144: F, t3137: F, t674: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t11523 = t5541 * t11522;
    let t11524 = t11523 * t8677;
    let t11526 = t5462 * t11522;
    let t11527 = t11526 * t8681;
    let t11529 = t3670 * t620;
    let t11532 = t190 * t8448;
    let t11533 = t11532 * t1;
    let t11534 = t116 * t11533;
    let t11535 = t11534 * t3703;
    let t11537 = t116 * t612;
    let t11539 = t3137 * t144 * t674;
    (t11523, t11524, t11526, t11527, t11529, t11533, t11534, t11535, t11537, t11539)
}
