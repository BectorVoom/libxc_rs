//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2787/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2787<F: Float>(t820: F, t823: F, t9948: F, t839: F, t10841: F, t10845: F, t10815: F, t2648: F, t2756: F, t2681: F, t2719: F, t2726: F) -> (F, F, F, F, F, F, F) {
    let t40360 = t820 * t823 * t9948;
    let t40361 = t40360 * t839;
    let t40374 = t10845 * t10841;
    let t40393 = t10815 * t2648;
    let t40395 = t10815 * t2756;
    let t40398 = t820 * t2719 * t2681;
    let t40399 = t40398 * t2726;
    (t40360, t40361, t40374, t40393, t40395, t40398, t40399)
}
