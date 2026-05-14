//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 558/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk558<F: Float>(t1937: F, t322: F, t449: F, t316: F, t1308: F, t1614: F, t1220: F, t1914: F, t119: F, t1907: F, t4137: F, t557: F, t1938: F, t857: F, t1782: F, t360: F) -> (F, F, F, F, F, F, F, F, F) {
    let t5510 = t1937 * t322;
    let t5511 = t449 * t5510;
    let t5512 = t316 * t5511;
    let t5514 = t1308 * t1614;
    let t5517 = t1220 * t1914 * t322;
    let t5518 = t316 * t5517;
    let t5520 = t119 * t1907;
    let t5523 = t4137 * t557;
    let t5525 = t857 * t1938;
    let t5527 = t1782 * t360;
    (t5511, t5512, t5514, t5517, t5518, t5520, t5523, t5525, t5527)
}
