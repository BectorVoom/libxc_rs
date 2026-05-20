//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta145 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk747;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk748;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta145<F: Float>(t3153: F, t3154: F, t3152: F, t1042: F, t1036: F, t3148: F, t3141: F, t357: F, t1038: F, t1052: F, t1033: F, t127: F, t246: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t3155, t3156, t3157, t3160, t3161, t3162, t3163, t3164, t3168, t3169) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk747::<F>(t3153, t3154, t3152, t1042, t1036, t3148, t3141, t357, t1038, t1052, t1033);
        let t3172 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk748::<F>(t127, t246);
    (t3155, t3156, t3157, t3160, t3161, t3162, t3163, t3164, t3168, t3169, t3172)
}
