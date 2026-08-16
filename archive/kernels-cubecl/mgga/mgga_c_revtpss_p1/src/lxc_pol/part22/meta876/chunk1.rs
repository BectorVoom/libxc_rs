//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3042/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3042<F: Float>(t4522: F, t874: F, t9288: F, t1573: F, t40317: F, t14587: F, t2782: F, t39608: F, t10069: F, t14496: F, t14524: F, t39575: F) -> (F, F, F, F, F) {
    let t51445 = t874 * t4522 * t9288;
    let t51452 = t40317 * t1573;
    let t51460 = t2782 * t39608 * t14587;
    let t51470 = t10069 * t14496;
    let t51483 = t39575 * t14524;
    (t51445, t51452, t51460, t51470, t51483)
}
