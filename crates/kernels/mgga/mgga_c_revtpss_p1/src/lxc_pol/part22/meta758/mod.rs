//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta758 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2837;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2838;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta758<F: Float>(t11626: F, t358: F, t3145: F, t3153: F, t3154: F, t11988: F, t3188: F, t11263: F, t3124: F, t11262: F, t3150: F, t3156: F, t3161: F, t3163: F, t3147: F, t3229: F, t3141: F, t3144: F, t1036: F, t11671: F, t3278: F, t2434: F, t246: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t42862, t42865, t42871, t42872, t42907, t42926, t42929) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2837::<F>(t11626, t358, t3145, t3153, t3154, t11988, t3188, t11263, t3124, t11262, t3150, t3156);
        let (t42932, t42939, t42943, t42967, t42994) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2838::<F>(t11262, t3161, t3163, t3147, t3229, t3141, t3144, t1036, t11671, t3278, t2434, t246);
    (t42862, t42865, t42871, t42872, t42907, t42926, t42929, t42932, t42939, t42943, t42967, t42994)
}
