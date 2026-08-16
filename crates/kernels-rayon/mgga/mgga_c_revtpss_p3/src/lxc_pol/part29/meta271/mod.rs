//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta271 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1120;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1121;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1122;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta271(t532: f64, t7535: f64, t1450: f64, t2107: f64, t7315: f64, t118: f64, t1310: f64, t1453: f64, t2014: f64, t2052: f64, t2056: f64, t2089: f64, t2093: f64, t2108: f64, t2322: f64, t4254: f64, t508: f64, t569: f64, t649: f64, t651: f64, t671: f64, t7235: f64, t7357: f64, t7359: f64, t7367: f64, t7374: f64, t7378: f64, t7474: f64, t7484: f64, t7489: f64, t3: f64, t116: f64, t2055: f64, param_d: f64, t670: f64, t117: f64, t7373: f64, t1459: f64, t1461: f64, t2113: f64, t2115: f64, t572: f64, t573: f64, t38: f64, t4173: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t7536, t7537, t7539, t7541) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1120(t532, t7535, t1450, t2107, t7315, t118, t1310, t1453, t2014, t2052, t2056, t2089, t2093, t2108, t2322, t4254, t508, t569, t649, t651, t671, t7235, t7357, t7359, t7367, t7374, t7378, t7474, t7484, t7489);
        let (t7542, t7547, t7553) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1121(t3, t7541, t116, t2055, param_d);
        let (t7554, t7557, t7560, t7702) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1122(t670, t7553, t117, t7373, t1459, t1461, t2113, t2115, t572, t573, t7547, t38, t4173);
    (t7536, t7537, t7539, t7541, t7542, t7547, t7553, t7554, t7557, t7560, t7702)
}
