//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta178 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk843;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta178<F: Float>(t187: F, t3850: F, t2608: F, t520: F, t512: F, t189: F, t19: F, t27: F, t521: F, t14: F, t22: F, t583: F, t588: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t3852, t3853, t3854, t3855, t3856, t3857, t3859, t3860, t3862, t3863) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk843::<F>(t187, t3850, t2608, t520, t512, t189, t19, t27, t521, t14, t22, t583, t588);
    (t3852, t3853, t3854, t3855, t3856, t3857, t3859, t3860, t3862, t3863)
}
