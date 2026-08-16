//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta177 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk893;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk894;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk895;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk896;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta177(t3863: f64, t521: f64, t1320: f64, t1333: f64, t198: f64, t2522: f64, t2562: f64, t2569: f64, t2579: f64, t2587: f64, t3827: f64, t3828: f64, t3829: f64, t3852: f64, t3854: f64, t3856: f64, t3859: f64, t3862: f64, t123: f64, t520: f64, t30: f64, t33: f64, t2630: f64, t1337: f64, t2619: f64, t514: f64, t1344: f64, t2257: f64, t3834: f64, t517: f64, t1348: f64, t3351: f64, t3842: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3865, t3867, t3868) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk893(t3863, t521, t1320, t1333, t198, t2522, t2562, t2569, t2579, t2587, t3827, t3828, t3829, t3852, t3854, t3856, t3859, t3862);
        let t3869 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk894(t123, t520);
        let (t3871, t3873, t3874, t3880, t3881, t3887) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk895(t30, t33, t2630, t3869, t1337, t2619, t514, t1344, t2257, t3834, t517, t1348, t3351, t3842, zeta_threshold);
        let t3889 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk896(t3880, t3887);
    (t3865, t3867, t3868, t3869, t3871, t3873, t3874, t3881, t3889)
}
