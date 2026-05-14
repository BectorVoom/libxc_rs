//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 952/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk952<F: Float>(t824: F, t8276: F, t2888: F, t2226: F, t3236: F, t1238: F, t2402: F, t1208: F, t6230: F, t2297: F, t6233: F, t898: F, t7920: F, t7922: F, t7924: F, t7926: F, t8006: F, t8008: F, t8011: F, t8015: F, t8019: F, t8023: F, t8025: F, t8027: F, t8030: F, t8034: F, t8241: F, t8243: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t8277 = t8276 * t824;
    let t8278 = t2888 * t8277;
    let t8281 = t3236 * t2226;
    let t8282 = t2888 * t8281;
    let t8285 = t1238 * t2402;
    let t8287 = t6230 * t1208;
    let t8288 = t6233 * t2297;
    let t8289 = t8287 * t8288;
    let t8291 = 0.10254018858216406658e4 * t898 * t8289;
    let t8292 = -t7920 - t7922 + t7924 + t7926 + t8006 - t8008 - t8011 - t8015 + t8019 + t8023 - t8025 - t8027 - t8030 + t8034 + t8241 + t8243 - t8291;
    (t8277, t8278, t8281, t8282, t8285, t8287, t8288, t8289, t8291, t8292)
}
