//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 946/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk946<F: Float>(t552: F, t7722: F, t32749: F, t6883: F, t32748: F, t6897: F, t794: F, t32762: F, t214: F, t32761: F, t114172: F, t7700: F) -> (F, F, F, F, F, F, F) {
    let t120492 = t552 * t7722;
    let t120514 = t6883 * t32749;
    let t120521 = t6897 * t794 * t32748;
    let t120532 = t6883 * t32762;
    let t120544 = t214 * t7722;
    let t120550 = t6897 * t794 * t32761;
    let t120568 = t6897 * t114172 * t7700;
    (t120492, t120514, t120521, t120532, t120544, t120550, t120568)
}
