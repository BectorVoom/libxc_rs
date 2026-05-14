//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 982/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk982<F: Float>(t2718: F, t860: F, t243: F, t816: F, t9707: F, t813: F, t2689: F, t2694: F, t247: F, t9949: F, t237: F, t236: F, t9646: F, t9721: F, t268: F, t207: F, t242: F) -> (F, F, F, F, F, F) {
    let t10661 = t2718 * t860;
    let t10671 = t9707 * t243 * t816;
    let t10673 = 0.12846167376791569079e-2 * t813 * t10671;
    let t10678 = t2689 * t2694;
    let t10685 = t9949 * t243 * t247;
    let t10687 = 0.37792653007779990369e-1 * t237 * t10685;
    let t10688 = t9646 * t236;
    let t10689 = t9721 * t243;
    let t10690 = t10689 * t268;
    let t10692 = 0.20082057720118594944e-6 * t10688 * t10690;
    let t10696 = 1.0 / t242 / t207;
    (t10661, t10673, t10678, t10687, t10692, t10696)
}
