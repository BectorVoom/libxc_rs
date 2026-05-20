//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta175 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk766;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk767;
use chunk2::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk768;
use chunk3::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk769;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta175<F: Float>(t3863: F, t521: F, t1320: F, t1333: F, t198: F, t2522: F, t2562: F, t2569: F, t2579: F, t2587: F, t3827: F, t3828: F, t3829: F, t3852: F, t3854: F, t3856: F, t3859: F, t3862: F, t123: F, t520: F, t30: F, t33: F, t2630: F, t1337: F, t2619: F, t514: F, t1344: F, t2257: F, t3834: F, t517: F, t1348: F, t3351: F, t3842: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t3865, t3867, t3868) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk766::<F>(t3863, t521, t1320, t1333, t198, t2522, t2562, t2569, t2579, t2587, t3827, t3828, t3829, t3852, t3854, t3856, t3859, t3862);
        let t3869 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk767::<F>(t123, t520);
        let (t3871, t3873, t3874, t3880, t3881, t3887) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk768::<F>(t30, t33, t2630, t3869, t1337, t2619, t514, t1344, t2257, t3834, t517, t1348, t3351, t3842, zeta_threshold);
        let t3889 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk769::<F>(t3880, t3887);
    (t3865, t3867, t3868, t3869, t3871, t3873, t3874, t3881, t3889)
}
