//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta281 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1256;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1257;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta281<F: Float>(t738: F, t745: F, t9385: F, t1340: F, t1320: F, t3853: F, t123: F, t147: F, t9291: F, t1317: F, t4029: F, t1333: F, t3863: F, t27: F, t583: F, t521: F, t19: F, t596: F, t182: F, t2490: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t9387, t9389, t9391, t9394) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1256::<F>(t738, t745, t9385, t1340, t1320, t3853, t123, t147, t9291);
        let (t9395, t9398, t9406, t9408, t9411, t9415, t9417) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1257::<F>(t1317, t3853, t1320, t4029, t1333, t3863, t27, t583, t521, t19, t596, t182, t2490);
    (t9387, t9389, t9391, t9394, t9395, t9398, t9406, t9408, t9411, t9415, t9417)
}
