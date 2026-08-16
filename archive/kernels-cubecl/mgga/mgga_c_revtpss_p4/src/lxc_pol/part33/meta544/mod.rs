//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta544 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1918;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1919;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta544<F: Float>(t1774: F, t7627: F, t7637: F, t1294: F, t8190: F, t7652: F, t1203: F, t8201: F, t1214: F, t8208: F, t2142: F, t5219: F, t1248: F, t1287: F, t1215: F, t26922: F, t26949: F, t26994: F, t29264: F, t29268: F, t29272: F, t29275: F, t5237: F, t5429: F, t5498: F, t7602: F, t7632: F, t7636: F, t7639: F, t7643: F, t7651: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t29278, t29279, t29283, t29287, t29293, t29297, t29301, t29304) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1918::<F>(t1774, t7627, t7637, t1294, t8190, t7652, t1203, t8201, t1214, t8208, t2142, t5219);
        let (t29308, t29311) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1919::<F>(t1248, t1287, t8208, t1215, t26922, t26949, t26994, t29264, t29268, t29272, t29275, t29279, t29283, t29287, t29293, t29297, t29301, t29304, t5237, t5429, t5498, t7602, t7632, t7636, t7639, t7643, t7651);
    (t29278, t29279, t29283, t29287, t29293, t29297, t29301, t29304, t29308, t29311)
}
