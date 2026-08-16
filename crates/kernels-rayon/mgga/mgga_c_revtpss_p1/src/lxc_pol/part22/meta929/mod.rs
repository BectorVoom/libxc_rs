//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta929 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3155;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3156;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta929(t127: f64, t12866: f64, t17650: f64, t5296: f64, t17861: f64, t3624: f64, t12784: f64, t17451: f64, t17416: f64, t3647: f64, t11262: f64, t1247: f64, t5286: f64, t17501: f64, t3172: f64, t3711: f64, t13099: f64, t43776: f64, t12956: f64, t17217: f64, t12909: f64, t17395: f64, t17384: f64, t12772: f64, t17668: f64, t3625: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t57098, t57100, t57114, t57118, t57125) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3155(t127, t12866, t17650, t5296, t17861, t3624, t12784, t17451, t17416, t3647, t11262, t1247, t5286);
        let (t57128, t57136, t57145, t57147, t57164, t57167) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3156(t17501, t3172, t3711, t13099, t43776, t12956, t17217, t12909, t17395, t12784, t17384, t12772, t17668, t3625);
    (t57098, t57100, t57114, t57118, t57125, t57128, t57136, t57145, t57147, t57164, t57167)
}
