//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta931 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3159;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3160;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta931(t17654: f64, t17657: f64, t56756: f64, t247: f64, t44545: f64, t5230: f64, t5384: f64, t12984: f64, t5327: f64, t12995: f64, t17438: f64, t17303: f64, t3667: f64, t12886: f64, t5381: f64, t12627: f64, t489: f64, t17728: f64, t13011: f64, t5373: f64, t1222: f64, t5368: f64, t697: f64, t17170: f64, t73: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t57227, t57241, t57250, t57252, t57256) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3159(t17654, t17657, t56756, t247, t44545, t5230, t5384, t12984, t5327, t12995, t17438, t17303, t3667);
        let (t57258, t57264, t57265, t57270, t57273, t57275) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3160(t12886, t5381, t12627, t489, t17728, t13011, t5373, t1222, t5368, t697, t17170, t73);
    (t57227, t57241, t57250, t57252, t57256, t57258, t57264, t57265, t57270, t57273, t57275)
}
