//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta283 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1261;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1262;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta283(t738: f64, t745: f64, t9385: f64, t1340: f64, t1320: f64, t3853: f64, t123: f64, t147: f64, t9291: f64, t1317: f64, t4029: f64, t3855: f64, t1333: f64, t3863: f64, t27: f64, t583: f64, t521: f64, t19: f64, t596: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9387, t9389, t9391, t9394) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1261(t738, t745, t9385, t1340, t1320, t3853, t123, t147, t9291);
        let (t9395, t9398, t9404, t9408, t9411, t9413) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1262(t1317, t3853, t1320, t4029, t3855, t1333, t3863, t27, t583, t521, t19, t596);
    (t9387, t9389, t9391, t9394, t9395, t9398, t9404, t9408, t9411, t9413)
}
