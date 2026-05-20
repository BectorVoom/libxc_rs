//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta563 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2021;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta563<F: Float>(t213: F, t25286: F, t251: F, t25304: F, t25374: F, t10505: F, t93172: F, t2453: F, t25398: F, t10506: F, t10982: F, t1949: F, t9646: F) -> (F, F, F, F, F, F, F, F) {
        let (t93186, t93189, t93190, t93191, t93192, t93194, t93195, t93206) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2021::<F>(t213, t25286, t251, t25304, t25374, t10505, t93172, t2453, t25398, t10506, t10982, t1949, t9646);
    (t93186, t93189, t93190, t93191, t93192, t93194, t93195, t93206)
}
