//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1038 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3628;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3629;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1038(t20473: f64, t3531: f64, t16685: f64, t5192: f64, t16652: f64, t57854: f64, t1196: f64, t12500: f64, t20472: f64, t20892: f64, t20384: f64, t3497: f64, t45187: f64, t45190: f64, t6518: f64, t16784: f64, t5198: f64, t12571: f64, t6548: f64, t1149: f64, t56265: f64, t57795: f64, t17151: f64, t5197: f64, t16639: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t68707, t68709, t68711, t68714, t68716, t68718, t68723) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3628(t20473, t3531, t16685, t5192, t16652, t57854, t1196, t12500, t20472, t20892, t20384, t3497, t45187, t45190, t6518);
        let (t68725, t68727, t68730, t68733, t68735) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3629(t16784, t5198, t12571, t6548, t1149, t56265, t57795, t1196, t17151, t5197, t16639, t5192);
    (t68707, t68709, t68711, t68714, t68716, t68718, t68723, t68725, t68727, t68730, t68733, t68735)
}
