//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1068/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1068<F: Float>(t3515: F, t759: F, t761: F, t2105: F, t2106: F, t3679: F, t287: F, t2916: F, t2900: F, t302: F, t3638: F, t5728: F) -> (F, F, F, F, F, F, F, F) {
    let t9268 = t3515 * t759;
    let t9269 = t9268 * t761;
    let t9270 = t2105 * t9269;
    let t9273 = t3679 * t2106;
    let t9274 = t2105 * t9273;
    let t9277 = t287 * t2916;
    let t9278 = t2900 * t9277;
    let t9279 = t302 * t9278;
    let t9282 = t3638 * t5728;
    (t9269, t9270, t9273, t9274, t9277, t9278, t9279, t9282)
}
