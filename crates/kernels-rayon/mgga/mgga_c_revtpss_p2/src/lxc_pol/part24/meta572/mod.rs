//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta572 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1751;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1752;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1753;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta572(t1196: f64, t12552: f64, t3523: f64, t90357: f64, t12248: f64, t6470: f64, t6474: f64, t1732: f64, t24324: f64, t3384: f64, t3433: f64, t81650: f64, t12227: f64, t20651: f64, t1765: f64, t82389: f64, t20400: f64, t6552: f64, t12254: f64, t141: f64, t89863: f64, t1145: f64, t89845: f64, t89853: f64, t89822: f64, t68255: f64, t68257: f64, t81156: f64, t81158: f64, t89839: f64, t89851: f64, t89865: f64, t89869: f64, t89873: f64, t89877: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t90361, t90364, t90367, t90370) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1751(t1196, t12552, t3523, t90357, t12248, t6470, t6474, t1732, t24324, t3384, t3433, t81650);
        let (t90373, t90375, t90377, t90379, t90384) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1752(t12227, t20651, t6470, t1765, t82389, t20400, t6552, t12254, t141, t89863, t1145, t89845);
        let (t90387, t90390, t90400) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1753(t1145, t141, t89853, t12254, t89822, t68255, t68257, t81156, t81158, t89839, t89851, t89865, t89869, t89873, t89877, t90379, t90384);
    (t90361, t90364, t90367, t90370, t90373, t90375, t90377, t90379, t90384, t90387, t90390, t90400)
}
