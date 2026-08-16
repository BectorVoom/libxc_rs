//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta841 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2716;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2717;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta841(t1261: f64, t12879: f64, t247: f64, t6425: f64, t12772: f64, t21227: f64, t3625: f64, t21021: f64, t21007: f64, t44425: f64, t21222: f64, t5340: f64, t21101: f64, t3707: f64, t17608: f64, t5292: f64, t17547: f64, t5265: f64, t20906: f64, t3172: f64, t17416: f64, t5391: f64, t21272: f64, t3636: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t70032, t70039, t70044, t70064, t70076) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2716(t1261, t12879, t247, t6425, t12772, t21227, t3625, t21021, t21007, t44425, t21222, t5340);
        let (t70082, t70088, t70091, t70102, t70112, t70114) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2717(t21101, t3707, t17608, t5292, t17547, t5265, t1261, t20906, t3172, t17416, t5391, t21272, t3636);
    (t70032, t70039, t70044, t70064, t70076, t70082, t70088, t70091, t70102, t70112, t70114)
}
