//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 856/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk856<F: Float>(t136: F, t243: F, t10815: F, t1561: F, t10845: F, t4430: F, t1558: F, t853: F, t4353: F, t9794: F, t10760: F, t10890: F, t1549: F) -> (F, F, F, F, F, F, F) {
    let t14685 = t243 * t136;
    let t14712 = t10815 * t1561;
    let t14716 = t10845 * t4430;
    let t14718 = t853 * t1558;
    let t14760 = t9794 * t4353;
    let t14761 = t10760 * t14760;
    let t14765 = t10890 * t1549;
    (t14685, t14712, t14716, t14718, t14760, t14761, t14765)
}
