//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta458 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1428;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1429;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta458(t1086: f64, t15669: f64, t3090: f64, t11629: f64, t53703: f64, t3316: f64, t4746: f64, t4891: f64, t1025: f64, t1663: f64, t2434: f64, t371: f64, t16170: f64, t372: f64, t11773: f64, t15925: f64, t1041: f64, t1670: f64, t42994: f64, t12046: f64, t1647: f64, t4995: f64, t3286: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t54500, t54564, t54570, t54687) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1428(t1086, t15669, t3090, t11629, t53703, t3316, t4746, t4891, t1025, t1663, t2434, t371);
        let (t55122, t55141, t55247, t55599, t55732, t55747) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1429(t16170, t372, t11773, t15925, t1041, t1670, t42994, t12046, t1647, t4746, t4995, t15669, t3286);
    (t54500, t54564, t54570, t54687, t55122, t55141, t55247, t55599, t55732, t55747)
}
