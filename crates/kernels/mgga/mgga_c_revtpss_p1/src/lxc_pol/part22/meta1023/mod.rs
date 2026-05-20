//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1023 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3573;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3574;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3575;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3576;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3577;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3578;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3579;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1023<F: Float>(t20294: F, t689: F, t12256: F, t2251: F, t5825: F, t12305: F, t128: F, t20297: F, t1120: F, t13312: F, t5051: F, t20319: F, t18281: F, t3362: F, t606: F, t3360: F, t43771: F, t43781: F, t43783: F, t43814: F, t43817: F, t68253: F, t68255: F, t68257: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t68262 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3573::<F>(t20294, t689);
        let (t68265, t68267) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3574::<F>(t12256, t2251, t5825, t12305, t128);
        let (t68269, t68271) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3575::<F>(t20297, t2251, t1120, t128);
        let (t68273, t68275) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3576::<F>(t13312, t5051, t1120, t128);
        let t68277 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3577::<F>(t20319, t689);
        let (t68280, t68282) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3578::<F>(t18281, t3362, t606, t128, t3360);
        let t68284 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3579::<F>(t43771, t43781, t43783, t43814, t43817, t68253, t68255, t68257, t68262, t68267, t68271, t68275, t68277, t68282);
    (t68262, t68265, t68267, t68269, t68271, t68273, t68275, t68277, t68280, t68282, t68284)
}
