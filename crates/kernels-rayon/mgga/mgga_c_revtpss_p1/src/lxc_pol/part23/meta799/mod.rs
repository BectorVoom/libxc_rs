//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta799 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2624;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2625;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta799(t40398: f64, t6024: f64, t18435: f64, t221: f64, t10703: f64, t2674: f64, t14832: f64, t2661: f64, t62351: f64, t775: f64, t10716: f64, t18423: f64, t62361: f64, t14648: f64, t4343: f64, t18398: f64, t2652: f64, t18415: f64, t9775: f64, t18410: f64, t18392: f64, t2675: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t62401, t62405, t62429, t62431) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2624(t40398, t6024, t18435, t221, t10703, t2674, t14832, t2661, t62351, t775, t10716, t18423);
        let (t62435, t62439, t62441, t62443, t62445, t62453) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2625(t14832, t2661, t62361, t775, t14648, t4343, t18398, t2652, t18415, t9775, t18410, t18392, t221, t2674, t2675);
    (t62401, t62405, t62429, t62431, t62435, t62439, t62441, t62443, t62445, t62453)
}
