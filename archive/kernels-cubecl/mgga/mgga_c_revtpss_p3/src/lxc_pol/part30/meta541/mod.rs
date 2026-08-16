//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta541 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1970;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta541<F: Float>(t29278: F, t7637: F, t1294: F, t8190: F, t7652: F, t1203: F, t8201: F, t1214: F, t8208: F, t2142: F, t5219: F, t1248: F, t1287: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t29279, t29282, t29283, t29287, t29292, t29293, t29296, t29297, t29300, t29301, t29304, t29308) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1970::<F>(t29278, t7637, t1294, t8190, t7652, t1203, t8201, t1214, t8208, t2142, t5219, t1248, t1287);
    (t29279, t29282, t29283, t29287, t29292, t29293, t29296, t29297, t29300, t29301, t29304, t29308)
}
