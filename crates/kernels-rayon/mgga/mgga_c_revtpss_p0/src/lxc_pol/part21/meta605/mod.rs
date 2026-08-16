//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta605 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2336;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2337;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta605(t10638: f64, t251: f64, t10111: f64, t22: f64, t2789: f64, t588: f64, t870: f64, t10963: f64, t9303: f64, t10069: f64, t10934: f64, t10518: f64, t10542: f64, t10612: f64, t2398: f64, t2434: f64, t2626: f64, t2629: f64, t676: f64, t9425: f64, t2567: f64, t2576: f64, t2582: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t39714, t39719, t39723, t39724, t39726, t39731) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2336(t10638, t251, t10111, t22, t2789, t588, t870, t10963, t9303, t10069, t10934, t10518, t10542);
        let (t39737, t39739, t39741, t39742, t39744, t39747) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2337(t10612, t2398, t2434, t2626, t2629, t676, t9425, t2567, t2576, t2582);
    (t39714, t39719, t39723, t39724, t39726, t39731, t39737, t39739, t39741, t39742, t39744, t39747)
}
