//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta596 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1928;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1929;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta596(t2470: f64, t28313: f64, t25387: f64, t95822: f64, t98892: f64, t95537: f64, t1957: f64, t26550: f64, t25372: f64, t98801: f64, t25386: f64, t2471: f64, t28373: f64, t10867: f64, t2061: f64, t14481: f64, t2062: f64, t2782: f64, t26519: f64, t99257: f64, t28341: f64, t786: f64, t789: f64, t10073: f64, t1579: f64, t2066: f64, t25390: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t103431, t103432, t103435, t103437, t103441, t103444, t103449) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1928(t2470, t28313, t25387, t95822, t98892, t95537, t1957, t26550, t25372, t98801, t25386, t2471, t28373);
        let (t103452, t103462, t103463, t103467, t103471) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1929(t10867, t2061, t14481, t2062, t2782, t26519, t99257, t28341, t786, t789, t10073, t1579, t2066, t25390);
    (t103431, t103432, t103435, t103437, t103441, t103444, t103449, t103452, t103462, t103463, t103467, t103471)
}
