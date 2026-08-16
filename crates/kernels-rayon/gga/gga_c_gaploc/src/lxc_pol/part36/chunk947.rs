//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 947/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk947(t43107: f64, t7290: f64, t1841: f64, t7289: f64, t40822: f64, t40825: f64, t40828: f64, t40833: f64, t40836: f64, t40850: f64, t40853: f64, t2508: f64, t2927: f64, t3266: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t43199 = t7290 * t43107;
    let t43202 = 0.17090058289204942852e-2_f64 * t1841 * t7289 * t43199;
    let t43203 = 0.1922631557535556071e-2_f64 * t40822;
    let t43204 = 0.3845263115071112142e-2_f64 * t40825;
    let t43205 = 0.1281754371690370714e-2_f64 * t40828;
    let t43206 = 0.2563508743380741428e-2_f64 * t40833;
    let t43207 = 0.64087718584518535698e-3_f64 * t40836;
    let t43208 = 0.1281754371690370714e-2_f64 * t40850;
    let t43209 = 0.64087718584518535698e-3_f64 * t40853;
    let t43212 = 0.76905262301422242837e-2_f64 * t2508 * t3266 * t2927;
    (t43199, t43202, t43203, t43204, t43205, t43206, t43207, t43208, t43209, t43212)
}
