//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta249 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1039;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1040;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1041;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta249<F: Float>(t1312: F, t1518: F, t4248: F, t5877: F, t5883: F, t5920: F, t93: F, t5545: F, t5547: F, t5570: F, t5572: F, t1907: F, t30: F, t33: F, t1468: F, t3833: F, t513: F, t5824: F, t1711: F, t3841: F, t516: F, t6416: F, t162: F, zeta_threshold: F, t189: F, t512: F, t1344: F, t3874: F, t1348: F, t3881: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t6773, t6777, t6778, t6779, t6780, t6781) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1039::<F>(t1312, t1518, t4248, t5877, t5883, t5920, t93, t5545, t5547, t5570, t5572, t1907);
        let (t6785, t6792, t6800) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1040::<F>(t30, t33, t1468, t3833, t513, t5824, t1711, t3841, t516, t6416, t162, zeta_threshold);
        let (t6801, t6802, t6816) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1041::<F>(t30, t33, t189, t6800, t512, t1344, t3874, t5824, t6785, t1348, t3881, t6416, t6792, zeta_threshold);
    (t6773, t6777, t6778, t6779, t6780, t6781, t6785, t6792, t6800, t6801, t6802, t6816)
}
