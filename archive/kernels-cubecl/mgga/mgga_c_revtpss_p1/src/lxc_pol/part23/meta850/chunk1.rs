//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2734/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2734<F: Float>(t3655: F, t6602: F, t20816: F, t3708: F, t17384: F, t17448: F, t17183: F, t17350: F, t17395: F, t5436: F, t17435: F, t5323: F) -> (F, F, F, F, F, F) {
    let t71192 = t6602 * t3655;
    let t71207 = t3708 * t20816;
    let t71232 = t17448 * t17384;
    let t71238 = t17183 * t17350;
    let t71275 = t5436 * t17395;
    let t71278 = t5323 * t17435;
    (t71192, t71207, t71232, t71238, t71275, t71278)
}
