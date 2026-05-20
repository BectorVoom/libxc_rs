//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1014/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1014<F: Float>(t2430: F, t854: F, t236: F, t807: F, t243: F, t247: F, t9949: F, t237: F, t9646: F, t9721: F, t268: F, t2479: F, t2652: F) -> (F, F, F, F, F, F, F, F) {
    let t10680 = t854 * t2430;
    let t10681 = t236 * t10680;
    let t10682 = t807 * t10681;
    let t10685 = t9949 * t243 * t247;
    let t10687 = F::cast_from(0.37792653007779990369e-1_f64) * t237 * t10685;
    let t10688 = t9646 * t236;
    let t10689 = t9721 * t243;
    let t10690 = t10689 * t268;
    let t10692 = F::cast_from(0.20082057720118594944e-6_f64) * t10688 * t10690;
    let t10693 = t2652 * t2479;
    (t10680, t10681, t10682, t10687, t10688, t10689, t10692, t10693)
}
