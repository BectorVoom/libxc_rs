//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1017/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1017<F: Float>(t3176: F, t68: F, t3174: F, t3026: F, t931: F, t824: F, t2888: F, t2226: F, t3236: F, t1238: F, t2402: F, t1208: F, t6230: F) -> (F, F, F, F, F, F, F, F) {
    let t8273 = t68 * t3176;
    let t8275 = t3174 * t8273 / F::new(72.0);
    let t8276 = t931 * t3026;
    let t8277 = t8276 * t824;
    let t8278 = t2888 * t8277;
    let t8281 = t3236 * t2226;
    let t8282 = t2888 * t8281;
    let t8285 = t1238 * t2402;
    let t8287 = t6230 * t1208;
    (t8275, t8276, t8277, t8278, t8281, t8282, t8285, t8287)
}
