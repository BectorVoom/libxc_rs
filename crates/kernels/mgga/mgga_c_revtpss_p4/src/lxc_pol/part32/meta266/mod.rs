//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta266 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1121;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1122;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1123;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta266<F: Float>(t532: F, t7535: F, t1450: F, t2107: F, t7315: F, t118: F, t1310: F, t1453: F, t2014: F, t2052: F, t2056: F, t2089: F, t2093: F, t2108: F, t2322: F, t4254: F, t508: F, t569: F, t649: F, t651: F, t671: F, t7235: F, t7357: F, t7359: F, t7367: F, t7374: F, t7378: F, t7474: F, t7484: F, t7489: F, t3: F, t116: F, t2055: F, param_d: F, t670: F, t117: F, t7373: F, t1459: F, t1461: F, t2113: F, t2115: F, t572: F, t573: F, t38: F, t4173: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t7536, t7537, t7539, t7541) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1121::<F>(t532, t7535, t1450, t2107, t7315, t118, t1310, t1453, t2014, t2052, t2056, t2089, t2093, t2108, t2322, t4254, t508, t569, t649, t651, t671, t7235, t7357, t7359, t7367, t7374, t7378, t7474, t7484, t7489);
        let (t7542, t7547, t7553) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1122::<F>(t3, t7541, t116, t2055, param_d);
        let (t7554, t7557, t7560, t7702) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1123::<F>(t670, t7553, t117, t7373, t1459, t1461, t2113, t2115, t572, t573, t7547, t38, t4173);
    (t7536, t7537, t7539, t7541, t7542, t7547, t7553, t7554, t7557, t7560, t7702)
}
