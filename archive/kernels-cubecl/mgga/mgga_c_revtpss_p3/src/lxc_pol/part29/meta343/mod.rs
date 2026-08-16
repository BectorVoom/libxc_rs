//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta343 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1264;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1265;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta343<F: Float>(t3241: F, t3244: F, t1058: F, t3197: F, t11132: F, t3163: F, t3172: F, t3161: F, t126: F, t373: F, t828: F, t3119: F, t3115: F, t1086: F, t3057: F, t3090: F, t11671: F, t3114: F, t127: F, t3206: F, t371: F, t3205: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t11886, t11888, t11890, t11917, t11921, t11922) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1264::<F>(t3241, t3244, t1058, t3197, t11132, t3163, t3172, t3161, t126, t373, t828);
        let (t11924, t11927, t11933, t11938) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1265::<F>(t11922, t3119, t3115, t1086, t3057, t3090, t11671, t3114, t127, t3206, t371, t3205);
    (t11886, t11888, t11890, t11917, t11921, t11922, t11924, t11927, t11933, t11938)
}
