//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta745 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2528;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2529;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta745(t10111: f64, t22: f64, t4518: f64, t231: f64, t39698: f64, t4494: f64, t10073: f64, t14509: f64, t10069: f64, t40921: f64, t4496: f64, t14537: f64, t10504: f64, t136: f64, t2457: f64, t4533: f64, t14473: f64, t9303: f64, t14477: f64, t2435: f64, t14482: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t51660, t51676, t51683, t51685, t51686, t51688) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2528(t10111, t22, t4518, t231, t39698, t4494, t10073, t14509, t10069, t40921, t4496, t14537);
        let (t51704, t51727, t51733, t51742, t51756) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2529(t10069, t14537, t10504, t136, t2457, t4533, t14473, t9303, t14477, t2435, t10073, t14482);
    (t51660, t51676, t51683, t51685, t51686, t51688, t51704, t51727, t51733, t51742, t51756)
}
