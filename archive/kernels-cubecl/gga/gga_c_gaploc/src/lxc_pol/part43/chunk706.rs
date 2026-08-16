//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 706/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk706<F: Float>(t13157: F, t1457: F, t6060: F, t1445: F, t2087: F, t10924: F, t2558: F, t9647: F, t1029: F, t3276: F, t2508: F, t3251: F, t9014: F) -> (F, F, F, F, F, F, F, F, F) {
    let t13158 = t1457 * t13157;
    let t13160 = F::cast_from(0.21450293971110256001e1_f64) * t6060 * t13158;
    let t13161 = t1445 * t13157;
    let t13163 = F::cast_from(0.62115540045351614476e2_f64) * t2087 * t13161;
    let t13182 = t10924 * t2558;
    let t13183 = t9647 * t13182;
    let t13184 = F::cast_from(0.64087718584518535698e-3_f64) * t13183;
    let t13185 = t3276 * t1029;
    let t13187 = F::cast_from(0.53833683610995569986e-1_f64) * t2508 * t13185;
    let t13191 = t9014 * t3251;
    (t13158, t13160, t13161, t13163, t13182, t13184, t13185, t13187, t13191)
}
