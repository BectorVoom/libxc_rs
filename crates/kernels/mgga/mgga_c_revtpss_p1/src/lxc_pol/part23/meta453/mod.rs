//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta453 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1883;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1884;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta453<F: Float>(t1089: F, t19477: F, t378: F, t3302: F, t357: F, t4866: F, t4893: F, t1071: F, t6299: F, t1043: F, t16560: F, t19450: F, t6258: F, t3153: F, t6305: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t19479, t19482, t19483, t19484, t19488, t19491, t19492) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1883::<F>(t1089, t19477, t378, t3302, t357, t4866, t4893, t1071, t6299, t1043, t16560, t19450);
        let (t19497, t19498, t19501) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1884::<F>(t1043, t6258, t1089, t3153, t6305);
    (t19479, t19482, t19483, t19484, t19488, t19491, t19492, t19497, t19498, t19501)
}
