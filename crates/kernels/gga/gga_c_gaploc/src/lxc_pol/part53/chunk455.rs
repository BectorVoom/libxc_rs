//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 455/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk455<F: Float>(t4529: F, t986: F, t2765: F, t524: F, t188: F, t7930: F, t493: F, t7892: F, t1339: F, t7905: F, t1397: F, t2897: F, t1359: F, t107: F, t7887: F, t544: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t8124 = t4529 * t986;
    let t8155 = t524 * t2765;
    let t8158 = t188 * t7930;
    let t8195 = t493 * t7892;
    let t8199 = t1339 * t7892;
    let t8207 = t493 * t7905;
    let t8229 = t1397 * t2897;
    let t8237 = t1359 * t986;
    let t8247 = t7887 * t107;
    let t8248 = t544 * t8247;
    (t8124, t8155, t8158, t8195, t8199, t8207, t8229, t8237, t8247, t8248)
}
