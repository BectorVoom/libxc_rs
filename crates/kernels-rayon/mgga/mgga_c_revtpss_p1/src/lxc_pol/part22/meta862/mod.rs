//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta862 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3013;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3014;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta862(t14547: f64, t14671: f64, t14686: f64, t50570: f64, t2661: f64, t2662: f64, t2754: f64, t4416: f64, t14738: f64, t2741: f64, t10845: f64, t14732: f64, t4423: f64, t853: f64, t2749: f64, t14718: f64, t14872: f64, t10777: f64, t10779: f64, t1548: f64, t14931: f64, t2724: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t50573, t50577, t50579, t50581) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3013(t14547, t14671, t14686, t50570, t2661, t2662, t2754, t4416, t14738, t2741, t10845, t14732);
        let (t50583, t50586, t50590, t50594, t50598) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3014(t4423, t853, t2661, t2662, t2749, t14718, t14872, t10777, t10779, t1548, t2754, t14671, t14686, t14931, t2724);
    (t50573, t50577, t50579, t50581, t50583, t50586, t50590, t50594, t50598)
}
