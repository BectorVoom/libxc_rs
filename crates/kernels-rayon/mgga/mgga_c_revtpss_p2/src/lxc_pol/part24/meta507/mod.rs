//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta507 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1517;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1518;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta507(t2723: f64, t2782: f64, t4503: f64, t76169: f64, t14568: f64, t18726: f64, t10871: f64, t14545: f64, t231: f64, t2783: f64, t76127: f64, t23359: f64, t822: f64, t213: f64, t262: f64, t5966: f64, t23148: f64, t23421: f64, t2411: f64, t11064: f64, t23429: f64, t892: f64, t23478: f64, t689: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t77177, t77183, t77191, t77197, t77225) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1517(t2723, t2782, t4503, t76169, t14568, t18726, t10871, t14545, t231, t2783, t76127, t23359, t822);
        let (t77316, t77333, t77341, t77357, t77373, t77460, t77499) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1518(t213, t23359, t262, t5966, t23148, t23421, t2411, t11064, t23429, t892, t23478, t689);
    (t77177, t77183, t77191, t77197, t77225, t77316, t77333, t77341, t77357, t77373, t77460, t77499)
}
