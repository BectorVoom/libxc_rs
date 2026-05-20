//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta573 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1989;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta573<F: Float>(t2453: F, t25309: F, t25301: F, t25304: F, t251: F, t25410: F, t136: F, t137: F, t1949: F, t2438: F, t837: F, t25305: F, t92894: F) -> (F, F, F, F, F, F, F, F) {
        let (t93158, t93161, t93169, t93170, t93172, t93174, t93175, t93177) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1989::<F>(t2453, t25309, t25301, t25304, t251, t25410, t136, t137, t1949, t2438, t837, t25305, t92894);
    (t93158, t93161, t93169, t93170, t93172, t93174, t93175, t93177)
}
