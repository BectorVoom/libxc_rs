//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta317 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1319;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1320;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta317<F: Float>(t159: F, t3181: F, t2851: F, t631: F, t45: F, t1071: F, t3057: F, t992: F, t338: F, t378: F, t3056: F, t988: F, t1031: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t11142, t11144, t11150, t11187, t11198, t11199, t11200) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1319::<F>(t159, t3181, t2851, t631, t45, t1071, t3057, t992, t338);
        let (t11201, t11223, t11224, t11238, t11239) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1320::<F>(t11200, t378, t3056, t988, t1031);
    (t11142, t11144, t11150, t11187, t11198, t11199, t11200, t11201, t11223, t11224, t11238, t11239)
}
