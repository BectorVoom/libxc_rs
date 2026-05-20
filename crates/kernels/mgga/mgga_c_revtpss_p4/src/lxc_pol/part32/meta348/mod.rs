//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta348 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1281;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1282;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta348<F: Float>(t10139: F, t14220: F, t13926: F, t543: F, t4100: F, t2782: F, t10014: F, t5741: F, t13790: F, t1398: F, t10022: F, t1892: F, t4086: F, t786: F, t4104: F) -> (F, F, F, F, F, F, F, F) {
        let (t14221, t14224) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1281::<F>(t10139, t14220, t13926, t543);
        let (t14227, t14229, t14230, t14233, t14239, t14241) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1282::<F>(t14224, t4100, t2782, t10014, t5741, t13790, t1398, t10022, t1892, t4086, t786, t4104);
    (t14221, t14224, t14227, t14229, t14230, t14233, t14239, t14241)
}
