//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 821/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk821<F: Float>(t2645: F, t2647: F, t9626: F, t210: F, t2553: F, t804: F, t2631: F, t828: F, t232: F, t819: F, t820: F, t2628: F, t835: F) -> (F, F, F, F, F, F) {
    let t9653 = t2645 * t9626 * t2647;
    let t9657 = t210 * t804 * t2553;
    let t9660 = t2631 * t828;
    let t9661 = t9660 * t232;
    let t9663 = t819 * t820 * t9661;
    let t9666 = t2628 * t835;
    (t9653, t9657, t9660, t9661, t9663, t9666)
}
