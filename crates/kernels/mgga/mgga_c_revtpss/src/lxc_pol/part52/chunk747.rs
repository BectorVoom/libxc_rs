//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 747/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk747<F: Float>(t532: F, t8713: F, t1450: F, t2014: F, t2033: F, t4147: F) -> (F, F, F, F) {
    let t8714 = t532 * t8713;
    let t8715 = t8714 * t1450;
    let t8716 = t2014 * t8715;
    let t8717 = t4147 * t2033;
    (t8714, t8715, t8716, t8717)
}
