//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2376/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2376<F: Float>(t10815: F, t2648: F, t2756: F, t2681: F, t2719: F, t820: F, t2726: F, t10850: F, t10861: F, t221: F, t2485: F, t10111: F, t823: F, t9720: F) -> (F, F, F, F, F) {
    let t40393 = t10815 * t2648;
    let t40395 = t10815 * t2756;
    let t40398 = t820 * t2719 * t2681;
    let t40399 = t40398 * t2726;
    let t40403 = t10850 * t2485 * t221 * t10861;
    let t40406 = t10111 * t823 * t9720;
    (t40393, t40395, t40399, t40403, t40406)
}
