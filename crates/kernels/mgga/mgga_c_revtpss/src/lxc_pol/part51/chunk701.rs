//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 701/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk701<F: Float>(t2033: F, t4147: F, t197: F, t532: F, t1450: F, t4146: F, t565: F) -> (F, F, F, F) {
    let t8717 = t4147 * t2033;
    let t8995 = t197 * t532;
    let t8996 = t2033 * t1450;
    let t9593 = 1.0 / t4146 / t565;
    (t8717, t8995, t8996, t9593)
}
