//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta837 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2708;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2709;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta837(t1160: f64, t20597: f64, t20447: f64, t3435: f64, t3565: f64, t6563: f64, t225: f64, t1261: f64, t12879: f64, t247: f64, t6429: f64, t11262: f64, t1247: f64, t6624: f64, t21102: f64, t3704: f64, t21094: f64, t3172: f64, t5384: f64, t17361: f64, t5274: f64, t5261: f64, t5390: f64, t12915: f64, t20703: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t69565, t69591, t69636, t69637, t69661, t69668) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2708(t1160, t20597, t20447, t3435, t3565, t6563, t225, t1261, t12879, t247, t6429, t11262, t1247, t6624);
        let (t69674, t69698, t69700, t69710, t69719) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2709(t21102, t3704, t21094, t3172, t5384, t17361, t5274, t5261, t5390, t12915, t20703, t247);
    (t69565, t69591, t69636, t69637, t69661, t69668, t69674, t69698, t69700, t69710, t69719)
}
