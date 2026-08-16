//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 946/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk946<F: Float>(t43107: F, t7290: F, t1841: F, t7289: F, t40822: F, t40825: F, t40828: F, t40833: F, t40836: F, t40850: F, t40853: F, t2508: F, t2927: F, t3266: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t43199 = t7290 * t43107;
    let t43202 = F::cast_from(0.17090058289204942852e-2_f64) * t1841 * t7289 * t43199;
    let t43203 = F::cast_from(0.1922631557535556071e-2_f64) * t40822;
    let t43204 = F::cast_from(0.3845263115071112142e-2_f64) * t40825;
    let t43205 = F::cast_from(0.1281754371690370714e-2_f64) * t40828;
    let t43206 = F::cast_from(0.2563508743380741428e-2_f64) * t40833;
    let t43207 = F::cast_from(0.64087718584518535698e-3_f64) * t40836;
    let t43208 = F::cast_from(0.1281754371690370714e-2_f64) * t40850;
    let t43209 = F::cast_from(0.64087718584518535698e-3_f64) * t40853;
    let t43212 = F::cast_from(0.76905262301422242837e-2_f64) * t2508 * t3266 * t2927;
    (t43199, t43202, t43203, t43204, t43205, t43206, t43207, t43208, t43209, t43212)
}
