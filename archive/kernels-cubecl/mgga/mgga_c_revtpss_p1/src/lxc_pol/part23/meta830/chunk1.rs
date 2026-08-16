//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2690/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2690<F: Float>(t11922: F, t16067: F, t19721: F, t19566: F, t3090: F, t1086: F, t19462: F, t19972: F, t4892: F, t19658: F, t3124: F, t19882: F, t3106: F) -> (F, F, F, F, F, F) {
    let t67526 = t16067 * t11922 * t19721;
    let t67528 = t19566 * t3090;
    let t67551 = t19462 * t1086 * t3090;
    let t67560 = t4892 * t11922 * t19972;
    let t67568 = t3124 * t19658;
    let t67571 = t3106 * t19882;
    (t67526, t67528, t67551, t67560, t67568, t67571)
}
