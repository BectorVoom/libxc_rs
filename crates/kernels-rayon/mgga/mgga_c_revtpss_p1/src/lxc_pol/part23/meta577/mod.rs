//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta577 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2186;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2187;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta577(t23168: f64, t827: f64, t828: f64, t23172: f64, t124: f64, t23114: f64, t800: f64, t23148: f64, t1544: f64, t5984: f64, t10673: f64, t10687: f64, t10692: f64, t10870: f64, t10900: f64, t14712: f64, t14716: f64, t14761: f64, t14765: f64, t18338: f64, t18340: f64, t2721: f64, t2730: f64, t799: f64, t5962: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t23253, t23257, t23262, t23263, t23266, t23267, t23275, t23278) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2186(t23168, t827, t828, t23172, t124, t23114, t800, t23148, t1544, t5984, t10673, t10687, t10692, t10870, t10900, t14712, t14716, t14761, t14765, t18338, t18340, t2721, t2730, t799);
        let t23279 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2187(t1544, t5962);
    (t23253, t23257, t23262, t23263, t23266, t23267, t23275, t23278, t23279)
}
