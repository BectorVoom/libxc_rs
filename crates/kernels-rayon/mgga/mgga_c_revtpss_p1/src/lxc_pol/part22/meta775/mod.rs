//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta775 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2863;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2864;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta775(t11262: f64, t3600: f64, t3605: f64, t3617: f64, t675: f64, t1261: f64, t247: f64, t3363: f64, t3609: f64, t44169: f64, t1263: f64, t215: f64, t1122: f64, t3711: f64, t3713: f64, t12657: f64, t1284: f64, t3624: f64, t221: f64, t461: f64, t462: f64, t624: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t44675, t44693, t44696, t44698, t44701) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2863(t11262, t3600, t3605, t3617, t675, t1261, t247, t3363, t3609, t44169, t1263, t215);
        let (t44704, t44751, t44769, t44797) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2864(t1122, t1261, t247, t44701, t11262, t3711, t3713, t12657, t1284, t3624, t221, t461, t462, t624);
    (t44675, t44693, t44696, t44698, t44701, t44704, t44751, t44769, t44797)
}
