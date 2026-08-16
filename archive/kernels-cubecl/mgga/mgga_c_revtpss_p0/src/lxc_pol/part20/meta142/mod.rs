//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta142 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk793;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk794;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk795;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk796;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk797;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk798;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta142<F: Float>(t30: F, t2257: F, t3833: F, t3834: F, t513: F, t527: F, t1113: F, zeta_threshold: F, t33: F, t3351: F, t516: F, t162: F, t187: F, t2608: F, t520: F, t512: F, t189: F, t19: F, t27: F, t521: F, t14: F, t22: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t3840, t3841, t3842) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk793::<F>(t30, t2257, t3833, t3834, t513, t527, t1113, zeta_threshold);
        let t3850 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk794::<F>(t33, t3351, t3841, t3842, t516, t162, t3840, zeta_threshold);
        let (t3852, t3853) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk795::<F>(t187, t3850, t2608, t520);
        let (t3854, t3855) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk796::<F>(t3853, t512, t189, t3850);
        let (t3856, t3857) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk797::<F>(t3855, t512, t19, t27);
        let (t3859, t3860) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk798::<F>(t3857, t521, t14, t22);
    (t3841, t3842, t3850, t3852, t3853, t3854, t3855, t3856, t3857, t3859, t3860)
}
