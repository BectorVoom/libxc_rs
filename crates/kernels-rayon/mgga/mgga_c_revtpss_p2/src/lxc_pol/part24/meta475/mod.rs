//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta475 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1458;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1459;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta475(t11509: f64, t6205: f64, t2967: f64, t6152: f64, t3011: f64, t6184: f64, t2942: f64, t2923: f64, t6104: f64, t3056: f64, t6234: f64, t378: f64, t1063: f64, t247: f64, t42447: f64, t6092: f64, t3140: f64, t6235: f64, t3149: f64, t11986: f64, t6100: f64, t11262: f64, t3161: f64, t6311: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t64043, t64060, t64125, t64319, t64336, t64686, t64687) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1458(t11509, t6205, t2967, t6152, t3011, t6184, t2942, t2923, t6104, t3056, t6234, t378);
        let (t65292, t65338, t65339, t65357, t65581) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1459(t1063, t247, t42447, t6092, t3140, t6235, t3149, t11986, t6100, t11262, t3161, t6311);
    (t64043, t64060, t64125, t64319, t64336, t64686, t64687, t65292, t65338, t65339, t65357, t65581)
}
