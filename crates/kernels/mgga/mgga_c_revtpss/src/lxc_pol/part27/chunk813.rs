//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 813/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk813<F: Float>(t10680: F, t236: F, t807: F, t243: F, t247: F, t9949: F, t237: F, t9646: F, t9721: F, t268: F, t2479: F, t2652: F, t207: F, t242: F, t240: F, t72: F) -> (F, F, F, F, F, F, F) {
    let t10681 = t236 * t10680;
    let t10682 = t807 * t10681;
    let t10685 = t9949 * t243 * t247;
    let t10687 = 0.37792653007779990369e-1 * t237 * t10685;
    let t10688 = t9646 * t236;
    let t10689 = t9721 * t243;
    let t10690 = t10689 * t268;
    let t10692 = 0.20082057720118594944e-6 * t10688 * t10690;
    let t10693 = t2652 * t2479;
    let t10696 = 1.0 / t242 / t207;
    let t10697 = t240 * t10696;
    let t10698 = t10697 * t72;
    (t10682, t10685, t10687, t10690, t10692, t10693, t10698)
}
