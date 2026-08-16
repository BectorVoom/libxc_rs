//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta556 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1995;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1996;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta556(t198: f64, t206: f64, t7086: f64, t25373: f64, t25392: f64, t25386: f64, t268: f64, t41040: f64, t837: f64, t25372: f64, t25287: f64, t786: f64, t789: f64, t2829: f64, t689: f64, t7014: f64, t2435: f64, t25352: f64, t11015: f64, t7018: f64, t7048: f64, t822: f64, t25300: f64, t9285: f64, t25299: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t92819, t92838, t92841, t92843, t92844, t92847) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1995(t198, t206, t7086, t25373, t25392, t25386, t268, t41040, t837, t25372, t25287, t786, t789);
        let (t92856, t92858, t92861, t92864, t92868, t92870) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1996(t2829, t689, t7014, t2435, t25352, t11015, t7018, t7048, t822, t25300, t9285, t25299);
    (t92819, t92838, t92841, t92843, t92844, t92847, t92856, t92858, t92861, t92864, t92868, t92870)
}
