//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1234/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1234<F: Float>(t16655: F, t3384: F, t3385: F, t5108: F, t12248: F, t3435: F, t5104: F, t1149: F, t3433: F, t3427: F, t12230: F, t1732: F, t12227: F, t3520: F, t5180: F, t5206: F) -> (F, F, F, F, F, F) {
    let t16657 = 2.0 * t3384 * t16655;
    let t16658 = t5108 * t3385;
    let t16660 = 0.96491876992155210402e2 * t12248 * t16658;
    let t16661 = t5104 * t3435;
    let t16662 = t16661 * t1149;
    let t16664 = 0.32163958997385070134e2 * t3433 * t16662;
    let t16665 = t5108 * t3427;
    let t16667 = 0.16081979498692535067e2 * t3433 * t16665;
    let t16668 = t1732 * t12230;
    let t16669 = t16668 * t3385;
    let t16671 = 0.51726012919273400301e3 * t12227 * t16669;
    let t16672 = t3520 * t5180;
    let t16673 = t16672 * t5206;
    (t16657, t16660, t16664, t16667, t16671, t16673)
}
