//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta685 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2501;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2502;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta685<F: Float>(t12861: F, t12916: F, t3718: F, t11262: F, t3600: F, t3605: F, t1261: F, t12925: F, t3172: F, t12921: F, t3711: F, t3617: F, t675: F, t247: F, t3363: F, t1263: F, t215: F, t1122: F, t12772: F, t12846: F, t5331: F, t12776: F, t3625: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t44672, t44675, t44678, t44681, t44693) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2501::<F>(t12861, t12916, t3718, t11262, t3600, t3605, t1261, t12925, t3172, t12921, t3711, t3617, t675);
        let (t44696, t44701, t44704, t44711, t44726) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2502::<F>(t1261, t247, t3363, t44693, t1263, t215, t1122, t12772, t12846, t5331, t12776, t3625);
    (t44672, t44675, t44678, t44681, t44696, t44701, t44704, t44711, t44726)
}
