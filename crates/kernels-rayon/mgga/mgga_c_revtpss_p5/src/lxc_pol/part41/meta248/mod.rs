//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta248 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk942;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk943;
use chunk2::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk944;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta248(t1312: f64, t1518: f64, t4248: f64, t5877: f64, t5883: f64, t5920: f64, t93: f64, t5545: f64, t5547: f64, t5570: f64, t5572: f64, t1907: f64, t30: f64, t33: f64, t1468: f64, t3833: f64, t513: f64, t5824: f64, t1711: f64, t3841: f64, t516: f64, t6416: f64, t162: f64, zeta_threshold: f64, t189: f64, t512: f64, t1344: f64, t3874: f64, t1348: f64, t3881: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t6773, t6777, t6778, t6779, t6780, t6781) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk942(t1312, t1518, t4248, t5877, t5883, t5920, t93, t5545, t5547, t5570, t5572, t1907);
        let (t6785, t6792, t6800) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk943(t30, t33, t1468, t3833, t513, t5824, t1711, t3841, t516, t6416, t162, zeta_threshold);
        let (t6801, t6802, t6816) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk944(t30, t33, t189, t6800, t512, t1344, t3874, t5824, t6785, t1348, t3881, t6416, t6792, zeta_threshold);
    (t6773, t6777, t6778, t6779, t6780, t6781, t6785, t6792, t6800, t6801, t6802, t6816)
}
