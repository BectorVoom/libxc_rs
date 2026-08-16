//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta758 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2837;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2838;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta758(t11626: f64, t358: f64, t3145: f64, t3153: f64, t3154: f64, t11988: f64, t3188: f64, t11263: f64, t3124: f64, t11262: f64, t3150: f64, t3156: f64, t3161: f64, t3163: f64, t3147: f64, t3229: f64, t3141: f64, t3144: f64, t1036: f64, t11671: f64, t3278: f64, t2434: f64, t246: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t42862, t42865, t42871, t42872, t42907, t42926, t42929) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2837(t11626, t358, t3145, t3153, t3154, t11988, t3188, t11263, t3124, t11262, t3150, t3156);
        let (t42932, t42939, t42943, t42967, t42994) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2838(t11262, t3161, t3163, t3147, t3229, t3141, t3144, t1036, t11671, t3278, t2434, t246);
    (t42862, t42865, t42871, t42872, t42907, t42926, t42929, t42932, t42939, t42943, t42967, t42994)
}
