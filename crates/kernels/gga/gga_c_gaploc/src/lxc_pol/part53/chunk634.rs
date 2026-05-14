//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 634/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk634<F: Float>(t13157: F, t1457: F, t6060: F, t1445: F, t2087: F, t10924: F, t2558: F, t9647: F, t1029: F, t3276: F, t2508: F, t3251: F, t9014: F, t10628: F, t5539: F, t10697: F, t3247: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t13158 = t1457 * t13157;
    let t13160 = 0.21450293971110256001e1 * t6060 * t13158;
    let t13161 = t1445 * t13157;
    let t13163 = 0.62115540045351614476e2 * t2087 * t13161;
    let t13182 = t10924 * t2558;
    let t13183 = t9647 * t13182;
    let t13184 = 0.64087718584518535698e-3 * t13183;
    let t13185 = t3276 * t1029;
    let t13187 = 0.53833683610995569986e-1 * t2508 * t13185;
    let t13191 = t9014 * t3251;
    let t13193 = 0.92286314761706691403e-1 * t2508 * t13191;
    let t13194 = t5539 * t10628;
    let t13195 = t9647 * t13194;
    let t13200 = t10697 * t3247;
    (t13158, t13160, t13161, t13163, t13182, t13184, t13185, t13187, t13191, t13193, t13194, t13195, t13200)
}
