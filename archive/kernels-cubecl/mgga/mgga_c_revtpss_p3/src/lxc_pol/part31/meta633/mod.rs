//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta633 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2087;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta633<F: Float>(t3201: F, t7801: F, t1058: F, t27467: F, t15775: F, t7132: F, t100054: F, t3299: F, t4857: F, t7125: F, t25495: F, t4845: F) -> (F, F, F, F, F, F) {
        let (t100272, t100275, t100289, t100302, t100324, t100327) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2087::<F>(t3201, t7801, t1058, t27467, t15775, t7132, t100054, t3299, t4857, t7125, t25495, t4845);
    (t100272, t100275, t100289, t100302, t100324, t100327)
}
