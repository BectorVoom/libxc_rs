//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta612 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2042;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2043;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta612<F: Float>(t25904: F, t97899: F, t1358: F, t212: F, t27960: F, t689: F, t26050: F, t27899: F, t2453: F, t27883: F, t25946: F, t27873: F, t94890: F, t136: F, t2457: F, t7929: F, t25944: F, t2470: F, t27887: F, t7284: F, t1955: F, t27836: F, t4075: F, t26072: F, t27888: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t97900, t97908, t97915, t97917, t97920) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2042::<F>(t25904, t97899, t1358, t212, t27960, t689, t26050, t27899, t2453, t27883, t25946, t27873, t94890);
        let (t97922, t97923, t97925, t97926, t97933, t97943) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2043::<F>(t136, t2457, t7929, t25944, t2470, t27887, t7284, t1955, t27836, t4075, t26072, t27888);
    (t97900, t97908, t97915, t97917, t97920, t97922, t97923, t97925, t97926, t97933, t97943)
}
