//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta377 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1270;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1271;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta377(t476: f64, t52: f64, t475: f64, t467: f64, t1785: f64, t6594: f64, t12678: f64, t16706: f64, t20283: f64, t20285: f64, t20287: f64, t24230: f64, t24234: f64, t24238: f64, t24242: f64, t24246: f64, t24250: f64, t459: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t24677, t24679, t24680, t24681, t24684, t24697) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1270(t476, t52, t475, t467, t1785, t6594, t12678, t16706, t20283, t20285, t20287, t24230, t24234, t24238, t24242, t24246, t24250);
        let t24698 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1271(t24697, t459);
    (t24677, t24679, t24680, t24681, t24684, t24697, t24698)
}
