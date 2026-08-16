//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta743 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2524;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2525;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta743(t51483: f64, t10069: f64, t14588: f64, t10518: f64, t14606: f64, t10073: f64, t14504: f64, t14575: f64, t2435: f64, t14568: f64, t1568: f64, t4503: f64, t786: f64, t40270: f64, t4496: f64, t136: f64, t137: f64, t14597: f64, t2438: f64, t2723: f64, t49180: f64, t836: f64, t2457: f64, t2710: f64, t4469: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t51484, t51507, t51513, t51522, t51538, t51547, t51548) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2524(t51483, t10069, t14588, t10518, t14606, t10073, t14504, t14575, t2435, t14568, t1568, t4503);
        let (t51549, t51553, t51561, t51564) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2525(t51548, t786, t40270, t4496, t136, t137, t14597, t2438, t2723, t49180, t836, t2457, t2710, t4469);
    (t51484, t51507, t51513, t51522, t51538, t51547, t51548, t51549, t51553, t51561, t51564)
}
