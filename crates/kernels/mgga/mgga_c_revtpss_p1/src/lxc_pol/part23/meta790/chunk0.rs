//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2605/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2605<F: Float>(t10845: F, t18531: F, t18618: F, t2741: F, t18622: F, t6016: F, t853: F, t2661: F, t2662: F, t2749: F, t14718: F, t18637: F) -> (F, F, F, F, F, F) {
    let t61572 = t10845 * t18531;
    let t61574 = t2741 * t18618;
    let t61576 = t10845 * t18622;
    let t61579 = t853 * t6016;
    let t61582 = t2661 * t2662 * t61579 * t2749;
    let t61612 = t2661 * t2662 * t14718 * t18637;
    (t61572, t61574, t61576, t61579, t61582, t61612)
}
