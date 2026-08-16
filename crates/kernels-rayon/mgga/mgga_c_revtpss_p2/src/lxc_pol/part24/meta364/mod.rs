//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta364 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1243;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1244;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1245;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta364(t1120: f64, t24248: f64, t128: f64, t12367: f64, t16706: f64, t20283: f64, t20285: f64, t20287: f64, t24230: f64, t24234: f64, t24238: f64, t24242: f64, t24246: f64, t448: f64, t300: f64, t1733: f64, t20629: f64, t5063: f64, t6471: f64, t16840: f64, t6474: f64, t24220: f64, t3435: f64, t12248: f64, t5071: f64, t6449: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t24249, t24250) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1243(t1120, t24248, t128);
        let (t24252, t24253) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1244(t12367, t16706, t20283, t20285, t20287, t24230, t24234, t24238, t24242, t24246, t24250, t448);
        let (t24255, t24257, t24259, t24261, t24262, t24264, t24265) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1245(t24253, t300, t1733, t20629, t5063, t6471, t16840, t6474, t24220, t3435, t12248, t5071, t6449);
    (t24249, t24250, t24252, t24253, t24255, t24257, t24259, t24261, t24262, t24264, t24265)
}
