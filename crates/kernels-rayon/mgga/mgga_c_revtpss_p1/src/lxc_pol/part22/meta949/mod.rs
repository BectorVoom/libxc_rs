//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta949 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3189;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3190;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta949(t29048: f64, t3362: f64, t3655: f64, t5258: f64, t5262: f64, t12976: f64, t5362: f64, t12963: f64, t5327: f64, t12995: f64, t17308: f64, t17283: f64, t3678: f64, t12901: f64, t17572: f64, t17235: f64, t372: f64, t13068: f64, t5292: f64, t1032: f64, t1246: f64, t17331: f64, t1247: f64, t17221: f64, t3172: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t59330, t59336, t59338, t59349, t59351, t59353, t59358) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3189(t29048, t3362, t3655, t5258, t5262, t12976, t5362, t12963, t5327, t12995, t17308, t17283, t3678);
        let (t59360, t59362, t59371, t59375, t59379) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3190(t12901, t17572, t17235, t372, t13068, t5292, t1032, t1246, t17331, t1247, t17221, t3172);
    (t59330, t59336, t59338, t59349, t59351, t59353, t59358, t59360, t59362, t59371, t59375, t59379)
}
