//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta143 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk727;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta143<F: Float>(t3853: F, t512: F, t19: F, t27: F, t521: F, t14: F, t22: F, t583: F, t588: F, t1320: F, t1333: F, t123: F, t520: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t3854, t3857, t3859, t3860, t3862, t3863, t3865, t3867, t3869) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk727::<F>(t3853, t512, t19, t27, t521, t14, t22, t583, t588, t1320, t1333, t123, t520);
    (t3854, t3857, t3859, t3860, t3862, t3863, t3865, t3867, t3869)
}
