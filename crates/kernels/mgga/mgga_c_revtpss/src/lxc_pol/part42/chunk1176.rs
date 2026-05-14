//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1176/1363 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1176<F: Float>(t19634: F, t6271: F, t3117: F, t12131: F, t357: F, t4786: F, t6100: F, t3092: F, t1065: F, t6244: F, t906: F, t1042: F, t3172: F, t6301: F, t1041: F, t5819: F, t606: F) -> (F, F, F, F, F, F) {
    let t19635 = t6271 * t19634;
    let t19636 = t3117 * t19635;
    let t19639 = t12131 * t357;
    let t19640 = t6271 * t19639;
    let t19641 = t3117 * t19640;
    let t19644 = t6100 * t4786;
    let t19645 = t3092 * t19644;
    let t19649 = t1065 * t6244;
    let t19650 = t19649 * t906;
    let t19651 = t1042 * t19650;
    let t19658 = t3172 * t6301;
    let t19659 = t1041 * t19658;
    let t19661 = t5819 * t606;
    (t19636, t19641, t19645, t19651, t19659, t19661)
}
