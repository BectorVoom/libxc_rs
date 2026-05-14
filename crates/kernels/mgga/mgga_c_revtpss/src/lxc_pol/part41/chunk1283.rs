//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1283/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1283<F: Float>(t5872: F, t644: F, t1469: F, t70: F, t72: F, t1927: F, t4186: F, t5819: F, t627: F, t19680: F, t18281: F, t36: F, t5826: F, t1486: F, t4181: F, t4187: F) -> (F, F, F, F, F, F, F, F, F) {
    let t21682 = t5872 * t644;
    let t21686 = t1469 * t70 * t72;
    let t21687 = t1927 * t4186;
    let t21690 = t5819 * t627;
    let t21695 = t19680 * t70;
    let t21698 = t36 * t18281;
    let t21699 = t21698 * t70;
    let t21702 = t5826 * t627;
    let t21707 = t4181 * t1486;
    let t21710 = t4187 * t1486;
    (t21682, t21686, t21687, t21690, t21695, t21699, t21702, t21707, t21710)
}
