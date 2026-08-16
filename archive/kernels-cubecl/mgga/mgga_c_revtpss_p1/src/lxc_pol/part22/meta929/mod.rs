//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta929 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3155;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3156;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta929<F: Float>(t127: F, t12866: F, t17650: F, t5296: F, t17861: F, t3624: F, t12784: F, t17451: F, t17416: F, t3647: F, t11262: F, t1247: F, t5286: F, t17501: F, t3172: F, t3711: F, t13099: F, t43776: F, t12956: F, t17217: F, t12909: F, t17395: F, t17384: F, t12772: F, t17668: F, t3625: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t57098, t57100, t57114, t57118, t57125) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3155::<F>(t127, t12866, t17650, t5296, t17861, t3624, t12784, t17451, t17416, t3647, t11262, t1247, t5286);
        let (t57128, t57136, t57145, t57147, t57164, t57167) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3156::<F>(t17501, t3172, t3711, t13099, t43776, t12956, t17217, t12909, t17395, t12784, t17384, t12772, t17668, t3625);
    (t57098, t57100, t57114, t57118, t57125, t57128, t57136, t57145, t57147, t57164, t57167)
}
