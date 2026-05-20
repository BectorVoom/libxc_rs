//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta325 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1328;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta325<F: Float>(t221: F, t346: F, t68: F, t345: F, t245: F, t3089: F, t3088: F, t3114: F, t11223: F, t225: F, t366: F, t1026: F, t371: F, t676: F) -> (F, F, F, F, F, F, F, F) {
        let (t11735, t11737, t11772, t11773, t11774, t11788, t11789, t11817) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1328::<F>(t221, t346, t68, t345, t245, t3089, t3088, t3114, t11223, t225, t366, t1026, t371, t676);
    (t11735, t11737, t11772, t11773, t11774, t11788, t11789, t11817)
}
