//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta328 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1331;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta328<F: Float>(t1086: F, t3057: F, t3090: F, t11671: F, t3114: F, t11200: F, t225: F, t1053: F, t3204: F, t1021: F, t3201: F, t1054: F) -> (F, F, F, F, F, F, F) {
        let (t11926, t11927, t11933, t11940, t11947, t11956, t11967) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1331::<F>(t1086, t3057, t3090, t11671, t3114, t11200, t225, t1053, t3204, t1021, t3201, t1054);
    (t11926, t11927, t11933, t11940, t11947, t11956, t11967)
}
