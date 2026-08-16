//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta865 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3019;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3020;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta865(t14860: f64, t2661: f64, t2662: f64, t837: f64, t2646: f64, t4352: f64, t14652: f64, t4416: f64, t14663: f64, t221: f64, t2484: f64, t2485: f64, t10811: f64, t14919: f64, t14904: f64, t14923: f64, t241: f64, t40322: f64, t820: f64, t2659: f64, t2783: f64, t816: f64, t808: f64, t853: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t50732, t50736, t50740, t50744, t50748) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3019(t14860, t2661, t2662, t837, t2646, t4352, t14652, t4416, t14663, t221, t2484, t2485);
        let (t50752, t50754, t50757, t50768, t50769) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3020(t10811, t14919, t14904, t14923, t241, t40322, t820, t2659, t2783, t816, t808, t853);
    (t50732, t50736, t50740, t50744, t50748, t50752, t50754, t50757, t50768, t50769)
}
