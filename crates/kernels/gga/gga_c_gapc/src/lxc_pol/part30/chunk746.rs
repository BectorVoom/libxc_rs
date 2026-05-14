//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 746/1135 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk746<F: Float>(t3424: F, t3427: F, t3431: F, t7675: F, t3437: F, t8915: F, t3440: F, t2660: F, t7880: F, t8655: F, t3434: F, t969: F, t1936: F, t928: F, t943: F, t3056: F) -> (F, F, F, F, F, F, F, F) {
    let t9766 = t3424 * t3427;
    let t9768 = t7675 * t3431;
    let t9770 = t3437 * t8915;
    let t9771 = t9770 * t3440;
    let t9775 = t2660 * t8655 * t7880;
    let t9777 = t3434 * t969;
    let t9779 = t928 * t1936;
    let t9780 = t9779 * t943;
    let t9782 = t928 * t3056;
    (t9766, t9768, t9770, t9771, t9775, t9777, t9780, t9782)
}
