//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta513 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2014;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2015;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2016;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta513(t21082: f64, t482: f64, t371: f64, t372: f64, t5323: f64, t5362: f64, t12772: f64, t6639: f64, t3625: f64, t1263: f64, t6573: f64, t1122: f64, t1042: f64, t1038: f64, t6593: f64, t1244: f64, t1241: f64, t5273: f64, t5292: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t21083, t21085, t21088, t21090, t21091, t21093, t21094) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2014(t21082, t482, t371, t372, t5323, t5362, t12772, t6639, t3625, t1263, t6573, t1122);
        let (t21095, t21101, t21102) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2015(t1042, t21094, t1038, t6593, t1244, t1241);
        let t21107 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2016(t5273, t5292);
    (t21083, t21085, t21088, t21090, t21091, t21093, t21094, t21095, t21101, t21102, t21107)
}
