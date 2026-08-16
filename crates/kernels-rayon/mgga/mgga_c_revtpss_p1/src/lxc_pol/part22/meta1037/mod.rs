//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1037 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3626;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3627;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1037(t20382: f64, t3520: f64, t1196: f64, t5206: f64, t12500: f64, t20895: f64, t5205: f64, t58000: f64, t1757: f64, t58708: f64, t68605: f64, t16662: f64, t57818: f64, t20394: f64, t3531: f64, t20896: f64, t12571: f64, t6556: f64, t20890: f64, t43977: f64, t68631: f64, t68633: f64, t68636: f64, t68640: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t68683, t68686, t68689, t68692, t68694) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3626(t20382, t3520, t1196, t5206, t12500, t20895, t5205, t58000, t1757, t58708, t68605, t16662, t57818);
        let (t68696, t68698, t68700, t68703, t68704) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3627(t20394, t3531, t20896, t12571, t6556, t1196, t20890, t43977, t68631, t68633, t68636, t68640, t68683, t68686, t68689, t68692, t68694);
    (t68683, t68686, t68689, t68692, t68694, t68696, t68698, t68700, t68703, t68704)
}
