//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2194/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2194<F: Float>(t2242: F, t5826: F, t19680: F, t603: F, t21663: F, t607: F, t5868: F, t644: F, t77: F, t13269: F, t1470: F, t4173: F, t4181: F) -> (F, F, F, F, F, F) {
    let t108762 = t2242 * t5826;
    let t108765 = t603 * t19680;
    let t108769 = t21663 * t607;
    let t108792 = t77 * t5868 * t644;
    let t108807 = t13269 * t1470;
    let t108810 = t4173 * t4181;
    (t108762, t108765, t108769, t108792, t108807, t108810)
}
