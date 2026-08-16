//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta458 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1428;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1429;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta458<F: Float>(t1086: F, t15669: F, t3090: F, t11629: F, t53703: F, t3316: F, t4746: F, t4891: F, t1025: F, t1663: F, t2434: F, t371: F, t16170: F, t372: F, t11773: F, t15925: F, t1041: F, t1670: F, t42994: F, t12046: F, t1647: F, t4995: F, t3286: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t54500, t54564, t54570, t54687) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1428::<F>(t1086, t15669, t3090, t11629, t53703, t3316, t4746, t4891, t1025, t1663, t2434, t371);
        let (t55122, t55141, t55247, t55599, t55732, t55747) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1429::<F>(t16170, t372, t11773, t15925, t1041, t1670, t42994, t12046, t1647, t4746, t4995, t15669, t3286);
    (t54500, t54564, t54570, t54687, t55122, t55141, t55247, t55599, t55732, t55747)
}
