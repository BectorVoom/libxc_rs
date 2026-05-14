//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 800/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk800<F: Float>(t7920: F, t914: F, t7925: F, t2731: F, t889: F, t155: F, t329: F, t7312: F, t2620: F, t947: F, t331: F, t7895: F, t2669: F, t875: F, t2608: F, t140: F, t7369: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8092 = t914 * t7920;
    let t8095 = t914 * t7925;
    let t8098 = t2731 * t889;
    let t8101 = t155 * t329 * t7312;
    let t8104 = t947 * t2620;
    let t8107 = 0.22391424203717421017e-2 * t331 * t7895;
    let t8108 = t2669 * t875;
    let t8109 = t8108 * t2608;
    let t8112 = t7369 * t140;
    (t8092, t8095, t8098, t8101, t8104, t8107, t8108, t8109, t8112)
}
