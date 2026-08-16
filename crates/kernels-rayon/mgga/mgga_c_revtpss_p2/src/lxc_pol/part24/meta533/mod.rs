//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta533 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1571;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1572;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta533(t1882: f64, t6843: f64, t22881: f64, t9962: f64, t6869: f64, t73856: f64, t9816: f64, t9818: f64, t2661: f64, t3992: f64, t74026: f64, t13999: f64, t22843: f64, t22854: f64, t3989: f64, t221: f64, t22852: f64, t3978: f64, t9921: f64, t22956: f64, t3930: f64, t22886: f64, t9744: f64, t13790: f64, t13845: f64, t13847: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t85659, t85705, t85735, t85741, t85752) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1571(t1882, t6843, t22881, t9962, t6869, t73856, t9816, t9818, t2661, t3992, t74026, t13999, t22843);
        let (t85764, t85778, t85782, t85791, t85816) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1572(t22854, t3989, t221, t22852, t3978, t9921, t22956, t3930, t22886, t9744, t13790, t13845, t13847, t73856);
    (t85659, t85705, t85735, t85741, t85752, t85764, t85778, t85782, t85791, t85816)
}
