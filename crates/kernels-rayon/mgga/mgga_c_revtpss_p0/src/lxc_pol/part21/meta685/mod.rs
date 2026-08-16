//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta685 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2501;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2502;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta685(t12861: f64, t12916: f64, t3718: f64, t11262: f64, t3600: f64, t3605: f64, t1261: f64, t12925: f64, t3172: f64, t12921: f64, t3711: f64, t3617: f64, t675: f64, t247: f64, t3363: f64, t1263: f64, t215: f64, t1122: f64, t12772: f64, t12846: f64, t5331: f64, t12776: f64, t3625: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t44672, t44675, t44678, t44681, t44693) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2501(t12861, t12916, t3718, t11262, t3600, t3605, t1261, t12925, t3172, t12921, t3711, t3617, t675);
        let (t44696, t44701, t44704, t44711, t44726) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2502(t1261, t247, t3363, t44693, t1263, t215, t1122, t12772, t12846, t5331, t12776, t3625);
    (t44672, t44675, t44678, t44681, t44696, t44701, t44704, t44711, t44726)
}
