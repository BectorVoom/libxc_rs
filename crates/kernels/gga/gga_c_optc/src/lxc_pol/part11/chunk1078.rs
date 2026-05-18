//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1078/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1078<F: Float>(t3546: F, t3563: F, t2045: F, t4580: F, t2048: F, t13110: F, t539: F, t1871: F, t40: F, t4579: F, t13004: F, t740: F) -> (F, F, F, F, F, F) {
    let t37422 = t3546 * t3563;
    let t37438 = t2045 * t4580;
    let t37441 = t2048 * t4580;
    let t37467 = t539 * t13110;
    let t37470 = t40 * t4579 * t1871;
    let t37498 = t13004 * t740;
    (t37422, t37438, t37441, t37467, t37470, t37498)
}
