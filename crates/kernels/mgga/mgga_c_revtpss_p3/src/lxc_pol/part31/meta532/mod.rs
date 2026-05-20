//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta532 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1912;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta532<F: Float>(t531: F, t7933: F, t7238: F, t2014: F, t1450: F, t5591: F, t7237: F, t13648: F, t2034: F, t25190: F, t7900: F, t5542: F, t7312: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t28172, t28173, t28175, t28176, t28177, t28179, t28182, t28183, t28184, t28186, t28187) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1912::<F>(t531, t7933, t7238, t2014, t1450, t5591, t7237, t13648, t2034, t25190, t7900, t5542, t7312);
    (t28172, t28173, t28175, t28176, t28177, t28179, t28182, t28183, t28184, t28186, t28187)
}
