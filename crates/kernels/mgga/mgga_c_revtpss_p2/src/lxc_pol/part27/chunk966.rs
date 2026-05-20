//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 966/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk966<F: Float>(t3133: F, t73: F, t3095: F, t3092: F, t2858: F, t4786: F, t3153: F, t4894: F, t3117: F, t4900: F, t2258: F, t3094: F) -> (F, F, F, F, F, F, F) {
    let t11678 = t3133 * t73;
    let t11679 = t11678 * t3095;
    let t11680 = t3092 * t11679;
    let t11683 = t2858 * t4786;
    let t11684 = t3092 * t11683;
    let t11687 = t3133 * t3153;
    let t11688 = t11687 * t4894;
    let t11689 = t3117 * t11688;
    let t11692 = t11687 * t4900;
    let t11693 = t3117 * t11692;
    let t11696 = t3094 * t2258;
    (t11678, t11680, t11684, t11687, t11689, t11693, t11696)
}
