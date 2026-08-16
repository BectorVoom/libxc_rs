//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta132 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk638;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk639;
use chunk2::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk640;
use chunk3::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk641;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta132<F: Float>(t2986: F, t315: F, t972: F, t973: F, t2846: F, t2904: F, t2848: F, t2855: F, t2860: F, t2864: F, t2882: F, t2890: F, t2898: F, t2900: F, t2906: F, t2910: F, t2913: F, t2916: F, t963: F, t323: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t2987, t2988) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk638::<F>(t2986, t315, t972);
        let (t2989, t2994, t3001, t3006) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk639::<F>(t2988, t973, t2846, t2904, t2848, t2855, t2860, t2864, t2882, t2890, t2898, t2900, t2906, t2910, t2913, t2916);
        let (t3007, t3010, t3011) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk640::<F>(t3006, t973, t963);
        let (t3012, t3013, t3014) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk641::<F>(t3011, t315, t323);
    (t2987, t2988, t2989, t2994, t3001, t3006, t3007, t3010, t3011, t3012, t3013, t3014)
}
