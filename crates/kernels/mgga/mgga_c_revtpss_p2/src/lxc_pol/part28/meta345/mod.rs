//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta345 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1365;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta345<F: Float>(t247: F, t2858: F, t3109: F, t1063: F, t140: F, t3247: F, t1011: F, t3254: F, t3237: F, t245: F, t3089: F, t3088: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t11744, t11745, t11752, t11753, t11755, t11756, t11762, t11763, t11772, t11773) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1365::<F>(t247, t2858, t3109, t1063, t140, t3247, t1011, t3254, t3237, t245, t3089, t3088);
    (t11744, t11745, t11752, t11753, t11755, t11756, t11762, t11763, t11772, t11773)
}
